// package main

// /*
// #include <stdlib.h>
// */
// import "C"
// import (
// 	"context"

// 	"github.com/pingcap/tidb/pkg/ddl/schematracker"
// 	"github.com/pingcap/tidb/pkg/parser"
// 	"github.com/pingcap/tidb/pkg/parser/ast"
// 	_ "github.com/pingcap/tidb/pkg/planner/core"
// 	_ "github.com/pingcap/tidb/pkg/types/parser_driver"
// 	"github.com/pingcap/tidb/pkg/util/collate"
// 	"github.com/pingcap/tidb/pkg/util/mock"
// )

// //export PrecheckSQL
// func PrecheckSQL(sql *C.char, collationEnabled C.int, verbose C.int) C.int {
// 	content := C.GoString(sql)
// 	stmts, _, err := parser.New().Parse(content, "", "")
// 	if err != nil || len(stmts) == 0 {
// 		return -1
// 	}
// 	collate.SetNewCollationEnabledForTest(collationEnabled != 0)
// 	tracker := schematracker.NewSchemaTracker(0)
// 	sessCtx := mock.NewContext()
// 	isLossyChange := false
// 	for _, stmt := range stmts {
// 		switch v := stmt.(type) {
// 		case *ast.CreateDatabaseStmt:
// 			err := tracker.CreateSchema(sessCtx, v)
// 			if err != nil {
// 				return -2
// 			}
// 		case *ast.CreateTableStmt:
// 			err := tracker.CreateTable(sessCtx, v)
// 			if err != nil {
// 				return -3
// 			}
// 		case *ast.AlterTableStmt:
// 			err := tracker.AlterTable(context.Background(), sessCtx, v)
// 			if err != nil {
// 				return -4
// 			}
// 			isLossyChange = tracker.Job.CtxVars[0].(bool)
// 		default:
// 			return -5
// 		}
// 	}
// 	if isLossyChange {
// 		return 1
// 	}
// 	return 0
// }

// func main() {}
