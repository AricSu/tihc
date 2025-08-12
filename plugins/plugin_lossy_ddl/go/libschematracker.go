package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"context"
	"fmt"
	"unsafe"

	"github.com/pingcap/log"
	"github.com/pingcap/tidb/pkg/ddl/schematracker"
	"github.com/pingcap/tidb/pkg/parser"
	"github.com/pingcap/tidb/pkg/parser/ast"
	_ "github.com/pingcap/tidb/pkg/planner/core"
	_ "github.com/pingcap/tidb/pkg/types/parser_driver"
	"github.com/pingcap/tidb/pkg/util/collate"
	"github.com/pingcap/tidb/pkg/util/mock"
)

// go build -buildmode=c-archive -o libschematracker.a libschematracker.go
func main() {
	// Empty main function required for c-archive build
}

// 屏蔽 TiDB 日志输出（只显示 error 及以上）
func supressLogOutput() {
	conf := new(log.Config)
	conf.Level = "error"
	lg, p, err := log.InitLogger(conf)
	if err == nil {
		log.ReplaceGlobals(lg, p)
	}
}

//export precheck_sql_c
func precheck_sql_c(sql_ptr *C.char, collation_enabled C.int, verbose_enabled C.int, error_msg **C.char) C.int {
	supressLogOutput() // 确保每次 FFI 调用都屏蔽日志
	sql := C.GoString(sql_ptr)
	collationEnabled := collation_enabled != 0
	isLossy, err := PrecheckSQL(sql, collationEnabled, verbose_enabled != 0)

	if err != nil {
		*error_msg = C.CString(err.Error())
		return -1 // error
	}

	*error_msg = nil // no error
	if isLossy {
		return 1 // lossy
	}
	return 0 // safe
}

//export free_error_message
func free_error_message(msg *C.char) {
	if msg != nil {
		C.free(unsafe.Pointer(msg))
	}
}

// PrecheckSQL 检查 SQL 是否包含有损 DDL 变更 (原生 Go 函数)
func PrecheckSQL(sql string, collationEnabled bool, verbose bool) (bool, error) {
	// 解析 SQL
	stmts, _, err := parser.New().Parse(sql, "", "")
	if err != nil {
		return false, fmt.Errorf("failed to parse SQL: %w", err)
	}
	if len(stmts) == 0 {
		return false, fmt.Errorf("no statements found")
	}

	// 检查最后一个语句必须是 ALTER TABLE（遵循原始 cmd 逻辑）
	if _, ok := stmts[len(stmts)-1].(*ast.AlterTableStmt); !ok {
		return false, fmt.Errorf("the last statement must be an ALTER TABLE statement")
	}

	// 设置 TiDB 组件 - 根据参数启用 collation 功能
	collate.SetNewCollationEnabledForTest(collationEnabled)
	tracker := schematracker.NewSchemaTracker(0)
	sessCtx := mock.NewContext()

	// 跟踪是否发现任何有损变更
	hasLossyChange := false

	// 处理每个语句
	for _, stmt := range stmts {
		switch v := stmt.(type) {
		case *ast.CreateDatabaseStmt:
			err := tracker.CreateSchema(sessCtx, v)
			if err != nil {
				return false, fmt.Errorf("failed to create schema: %w", err)
			}
		case *ast.CreateTableStmt:
			err := tracker.CreateTable(sessCtx, v)
			if err != nil {
				return false, fmt.Errorf("failed to create table: %w", err)
			}
		case *ast.AlterTableStmt:
			err := tracker.AlterTable(context.Background(), sessCtx, v)
			if err != nil {
				return false, fmt.Errorf("failed to alter table: %w", err)
			}

			// 检查是否有损变更 - 只有 ALTER TABLE 才检查
			if tracker.Job == nil {
				return false, fmt.Errorf("tracker job is nil after alter table operation")
			}
			if len(tracker.Job.CtxVars) == 0 {
				return false, fmt.Errorf("tracker job context variables is empty")
			}

			lossy, ok := tracker.Job.CtxVars[0].(bool)
			if !ok {
				return false, fmt.Errorf("failed to get lossy value from tracker job context")
			}

			// 累积有损变更信息，而不是立即返回
			if lossy {
				hasLossyChange = true
			}
		default:
			// 不支持的语句类型，返回错误
			return false, fmt.Errorf("unsupported statement type: %T", v)
		}

		// verbose 输出现在由 Rust 端的日志系统处理，这里不再直接打印
		// Go 端专注于计算逻辑，日志输出交给 Rust 端统一管理
	}

	// 返回累积的有损变更结果
	return hasLossyChange, nil
}
