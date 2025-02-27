use sqlparser::ast::{
    AssignmentTarget, Expr, FromTable, FunctionArg, FunctionArgExpr, FunctionArguments,
    GroupByExpr, Ident, Join, JoinConstraint, JoinOperator, ObjectName, OnInsert, Query,
    SelectItem, SetExpr, Statement, TableFactor, TableWithJoins, UpdateTableFromKind,
};
use tracing::{debug, warn};

pub struct ColumnReplacer;

impl ColumnReplacer {
    pub fn new() -> Self {
        debug!("Initializing ColumnReplacer");
        ColumnReplacer {}
    }

    pub fn apply(&mut self, statements: &mut Vec<Statement>) {
        for statement in statements {
            self.replace_column_names_in_statement(statement);
        }
    }

    fn replace_column_names_in_ident(&mut self, ident: &mut ObjectName) {
        for part in &mut ident.0 {
            part.value = "tihc".to_string();
        }
    }

    fn replace_column_names_in_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Identifier(Ident { value, .. }) => {
                debug!("Replacing Identifier: {}", value);
                *value = "tihc".to_string();
            }
            Expr::CompoundIdentifier(parts) => {
                debug!("Replacing CompoundIdentifier: {:?}", parts);
                for part in parts {
                    part.value = "tihc".to_string();
                }
            }
            Expr::BinaryOp { left, right, .. } => {
                debug!("Replacing BinaryOp: left = {:?}, right = {:?}", left, right);
                self.replace_column_names_in_expr(left);
                self.replace_column_names_in_expr(right);
            }
            Expr::UnaryOp { expr, .. } => {
                debug!("Replacing UnaryOp: {:?}", expr);
                self.replace_column_names_in_expr(expr);
            }
            Expr::Function(func) => {
                debug!("Replacing Function: {:?}", func);
                self.replace_function_arguments(&mut func.args);
            }
            Expr::Nested(expr) => {
                debug!("Replacing Nested: {:?}", expr);
                self.replace_column_names_in_expr(expr);
            }
            Expr::Subquery(query) => {
                self.replace_column_names_in_query(query);
            }
            Expr::Case {
                operand,
                conditions,
                results,
                else_result,
                ..
            } => {
                debug!("Replacing Case: operand = {:?}, conditions = {:?}, results = {:?}, else_result = {:?}", operand, conditions, results, else_result);
                if let Some(op) = operand {
                    self.replace_column_names_in_expr(op);
                }
                for condition in conditions {
                    self.replace_column_names_in_expr(condition);
                }
                for result in results {
                    self.replace_column_names_in_expr(result);
                }
                if let Some(else_result) = else_result {
                    self.replace_column_names_in_expr(else_result);
                }
            }
            Expr::InSubquery { expr, subquery, .. } => {
                debug!("Replacing InSubquery: {:?}", subquery);
                self.replace_column_names_in_expr(expr);
                self.replace_column_names_in_query(subquery);
            }
            _ => {
                warn!("Unhandled expression: {:?}", expr);
            }
        }
    }

    fn replace_function_arguments(&mut self, args: &mut FunctionArguments) {
        match args {
            FunctionArguments::None => {
                debug!("FunctionArguments::None");
            }
            FunctionArguments::Subquery(subquery) => {
                debug!("FunctionArguments::Subquery: {:?}", subquery);
                self.replace_column_names_in_query(subquery);
            }
            FunctionArguments::List(arg_list) => {
                debug!("FunctionArguments::List: {:?}", arg_list);
                for arg in &mut arg_list.args {
                    match arg {
                        FunctionArg::ExprNamed { ref mut arg, .. }
                        | FunctionArg::Unnamed(ref mut arg) => {
                            if let FunctionArgExpr::Expr(ref mut expr) = arg {
                                debug!("Replacing FunctionArgExpr: {:?}", expr);
                                self.replace_column_names_in_expr(expr);
                            }
                        }
                        _ => {
                            warn!("Unhandled function argument: {:?}", arg);
                        }
                    }
                }
            }
        }
    }

    fn replace_column_names_in_query(&mut self, query: &mut Query) {
        if let Some(order_by) = &mut query.order_by {
            for expr in &mut order_by.exprs {
                self.replace_column_names_in_expr(&mut expr.expr);
            }
        }

        self.replace_column_names_in_set_expr(&mut query.body);
    }

    fn replace_column_names_in_set_expr(&mut self, set_expr: &mut SetExpr) {
        match set_expr {
            SetExpr::Select(select) => {
                self.replace_select_items(&mut select.projection);
                self.replace_where_having_and_group_by(select);
                self.replace_from(&mut select.from);
            }
            SetExpr::Query(subquery) => {
                self.replace_column_names_in_query(subquery);
            }
            _ => {
                warn!("Ignoring unsupported SetExpr variant: {:?}", set_expr);
            }
        }
    }

    fn replace_select_items(&mut self, select_items: &mut [SelectItem]) {
        for select_item in select_items {
            if let SelectItem::UnnamedExpr(ref mut expr) = select_item {
                self.replace_column_names_in_expr(expr);
            }
        }
    }

    fn replace_where_having_and_group_by(&mut self, select: &mut sqlparser::ast::Select) {
        if let Some(where_expr) = &mut select.selection {
            self.replace_column_names_in_expr(where_expr);
        }
        if let Some(having_expr) = &mut select.having {
            self.replace_column_names_in_expr(having_expr);
        }
        if let GroupByExpr::Expressions(expressions, _) = &mut select.group_by {
            for expr in expressions {
                self.replace_column_names_in_expr(expr);
            }
        }
    }

    fn replace_from(&mut self, from: &mut Vec<TableWithJoins>) {
        for source in from {
            match &mut source.relation {
                TableFactor::Derived { subquery, .. } => {
                    debug!("Replacing columns in derived table");
                    self.replace_column_names_in_query(subquery);
                }
                TableFactor::NestedJoin {
                    table_with_joins, ..
                } => {
                    debug!("Replacing columns in nested join");
                    self.replace_column_names_in_table_with_joins(table_with_joins);
                }
                TableFactor::Table { name, alias, .. } => {
                    debug!("Replacing columns in table factor: {:?}", name);
                    if let Some(alias) = alias {
                        debug!("Table alias: {:?}", alias);
                    }
                    // Do not replace table names
                }
                _ => {
                    warn!("Unhandled table factor: {:?}", source.relation);
                }
            }
            for join in &mut source.joins {
                debug!("Replacing columns in join");
                self.replace_column_names_in_join(join);
            }
        }
    }

    // fn replace_select_items(&mut self, select_items: &mut [SelectItem]) {
    //     for select_item in select_items {
    //         if let SelectItem::UnnamedExpr(ref mut expr) = select_item {
    //             self.replace_column_names_in_expr(expr);
    //         }
    //     }
    // }

    // fn replace_where_having_group_by(&mut self, select: &mut sqlparser::ast::Select) {
    //     if let Some(where_expr) = &mut select.selection {
    //         self.replace_column_names_in_expr(where_expr);
    //     }
    //     if let Some(having_expr) = &mut select.having {
    //         self.replace_column_names_in_expr(having_expr);
    //     }
    //     if let GroupByExpr::Expressions(expressions, _) = &mut select.group_by {
    //         for expr in expressions {
    //             self.replace_column_names_in_expr(expr);
    //         }
    //     }
    // }

    // fn replace_from(&mut self, from: &mut Vec<TableWithJoins>) {
    //     for source in from {
    //         match &mut source.relation {
    //             TableFactor::Derived { subquery, .. } => {
    //                 debug!("Replacing columns in derived table");
    //                 self.replace_column_names_in_query(subquery);
    //             }
    //             TableFactor::NestedJoin { table_with_joins, .. } => {
    //                 debug!("Replacing columns in nested join");
    //                 self.replace_column_names_in_table_with_joins(table_with_joins);
    //             }
    //             TableFactor::Table { name, alias, .. } => {
    //                 debug!("Replacing columns in table factor: {:?}", name);
    //                 if let Some(alias) = alias {
    //                     debug!("Table alias: {:?}", alias);
    //                 }
    //                 // Do not replace table names
    //             }
    //             _ => {
    //                 warn!("Unhandled table factor: {:?}", source.relation);
    //             }
    //         }
    //         for join in &mut source.joins {
    //             debug!("Replacing columns in join");
    //             self.replace_column_names_in_join(join);
    //         }
    //     }
    // }

    fn replace_column_names_in_table_with_joins(&mut self, table_with_joins: &mut TableWithJoins) {
        debug!("Replacing columns in table with joins");
        self.replace_column_names_in_table_factor(&mut table_with_joins.relation);
        for join in &mut table_with_joins.joins {
            self.replace_column_names_in_join(join);
        }
    }

    fn replace_column_names_in_table_factor(&mut self, table_factor: &mut TableFactor) {
        match table_factor {
            TableFactor::Table {
                ref mut name,
                alias,
                ..
            } => {
                debug!("Replacing columns in table factor: {:?}", name);
                // Do not replace table names
                if let Some(alias) = alias {
                    debug!("Table alias: {:?}", alias);
                }
            }
            TableFactor::Derived { subquery, .. } => {
                debug!("Replacing columns in derived table factor");
                self.replace_column_names_in_query(subquery);
            }
            TableFactor::NestedJoin {
                table_with_joins, ..
            } => {
                debug!("Replacing columns in nested join table factor");
                self.replace_column_names_in_table_with_joins(table_with_joins);
            }
            _ => {
                warn!("Unhandled table factor: {:?}", table_factor);
            }
        }
    }

    fn replace_column_names_in_join(&mut self, join: &mut Join) {
        debug!("Replacing columns in join relation");
        self.replace_column_names_in_table_factor(&mut join.relation);
        match &mut join.join_operator {
            JoinOperator::Inner(constraint)
            | JoinOperator::LeftOuter(constraint)
            | JoinOperator::RightOuter(constraint)
            | JoinOperator::FullOuter(constraint)
            | JoinOperator::Semi(constraint)
            | JoinOperator::LeftSemi(constraint)
            | JoinOperator::RightSemi(constraint)
            | JoinOperator::Anti(constraint)
            | JoinOperator::LeftAnti(constraint)
            | JoinOperator::RightAnti(constraint) => {
                debug!("Replacing join constraint");
                self.replace_column_names_in_join_constraint(constraint);
            }
            _ => {
                warn!("Unhandled join operator: {:?}", join.join_operator);
            }
        }
    }

    fn replace_column_names_in_join_constraint(&mut self, constraint: &mut JoinConstraint) {
        match constraint {
            JoinConstraint::On(ref mut expr) => {
                debug!("Replacing columns in join constraint ON expression");
                self.replace_column_names_in_expr(expr);
            }
            JoinConstraint::Using(object_names) => {
                debug!("Replacing columns in join constraint USING expression");
                for name in object_names {
                    self.replace_column_names_in_ident(name);
                }
            }
            _ => {
                warn!("Unhandled join constraint: {:?}", constraint);
            }
        }
    }

    fn replace_column_names_in_statement(&mut self, statement: &mut Statement) {
        match statement {
            Statement::Insert(ref mut insert) => self.replace_insert_columns(insert),
            Statement::Update {
                assignments,
                selection,
                from,
                returning,
                ..
            } => {
                self.replace_update_columns(assignments);
                if let Some(expr) = selection {
                    self.replace_column_names_in_expr(expr);
                }
                if let Some(from_clause) = from.as_mut() {
                    self.replace_update_from(from_clause);
                }
                if let Some(returning_items) = returning {
                    for select_item in returning_items {
                        if let SelectItem::UnnamedExpr(ref mut expr) = select_item {
                            self.replace_column_names_in_expr(expr);
                        }
                    }
                }
            }
            Statement::Delete(delete) => {
                if let Some(expr) = &mut delete.selection {
                    self.replace_column_names_in_expr(expr);
                }
                match &mut delete.from {
                    FromTable::WithFromKeyword(from) | FromTable::WithoutKeyword(from) => {
                        self.replace_from(from);
                    }
                }
                if let Some(using) = &mut delete.using {
                    for table_with_joins in using {
                        self.replace_column_names_in_table_with_joins(table_with_joins);
                    }
                }
                if let Some(returning_items) = &mut delete.returning {
                    for select_item in returning_items {
                        if let SelectItem::UnnamedExpr(ref mut expr) = select_item {
                            self.replace_column_names_in_expr(expr);
                        }
                    }
                }
                for order_by_expr in &mut delete.order_by {
                    self.replace_column_names_in_expr(&mut order_by_expr.expr);
                }
            }
            Statement::Query(query) => self.replace_column_names_in_query(query),
            _ => warn!("Ignoring statement: {:?}", statement),
        }
    }

    fn replace_insert_columns(&mut self, insert: &mut sqlparser::ast::Insert) {
        for column in &mut insert.columns {
            column.value = "tihc".to_string();
        }
        for assignment in &mut insert.assignments {
            if let AssignmentTarget::ColumnName(ObjectName(parts)) = &mut assignment.target {
                for part in parts {
                    part.value = "tihc".to_string();
                }
            }
            self.replace_column_names_in_expr(&mut assignment.value);
        }
        if let Some(on_insert) = &mut insert.on {
            if let OnInsert::DuplicateKeyUpdate(assignments) = on_insert {
                for assignment in assignments {
                    self.replace_insert_assignment(assignment);
                }
            }
        }
    }

    fn replace_insert_assignment(&mut self, assignment: &mut sqlparser::ast::Assignment) {
        if let AssignmentTarget::ColumnName(ObjectName(parts)) = &mut assignment.target {
            for part in parts {
                part.value = "tihc".to_string();
            }
        }
        self.replace_column_names_in_expr(&mut assignment.value);
    }

    fn replace_update_columns(&mut self, assignments: &mut Vec<sqlparser::ast::Assignment>) {
        for assignment in assignments {
            if let AssignmentTarget::ColumnName(ObjectName(parts)) = &mut assignment.target {
                for part in parts {
                    part.value = "tihc".to_string();
                }
            }
            self.replace_column_names_in_expr(&mut assignment.value);
        }
    }

    fn replace_update_from(&mut self, from_clause: &mut UpdateTableFromKind) {
        match from_clause {
            UpdateTableFromKind::BeforeSet(table_with_joins)
            | UpdateTableFromKind::AfterSet(table_with_joins) => {
                self.replace_column_names_in_table_with_joins(table_with_joins);
            }
        }
    }
}
