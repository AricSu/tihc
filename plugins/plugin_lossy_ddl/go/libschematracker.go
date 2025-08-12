package main

import "C"

import (
	"context"
	"fmt"

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

//export precheck_sql_with_collation
func precheck_sql_with_collation(sql_ptr *C.char, collation_enabled C.int) C.int {
	sql := C.GoString(sql_ptr)
	collationEnabled := collation_enabled != 0
	isLossy, err := PrecheckSQL(sql, collationEnabled)
	if err != nil {
		return -1 // 错误情况
	}
	if isLossy {
		return 1 // 有损
	}
	return 0 // 安全
}

// PrecheckSQL 检查 SQL 是否包含有损 DDL 变更
func PrecheckSQL(sql string, collationEnabled bool) (bool, error) {
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
			if tracker.Job.CtxVars == nil || len(tracker.Job.CtxVars) == 0 {
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
	}

	// 返回累积的有损变更结果
	return hasLossyChange, nil
}
