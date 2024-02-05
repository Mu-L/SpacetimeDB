use super::util::fmt_fn;

use std::fmt::{self, Write};

use convert_case::{Case, Casing};
use nonempty::NonEmpty;
use spacetimedb_lib::sats::db::def::TableSchema;
use spacetimedb_lib::sats::{
    AlgebraicType, AlgebraicType::Builtin, AlgebraicTypeRef, ArrayType, BuiltinType, MapType, ProductType, SumType,
};
use spacetimedb_lib::{ReducerDef, TableDesc};

use super::code_indenter::CodeIndenter;
use super::{GenCtx, GenItem};

enum MaybePrimitive<'a> {
    Primitive(&'static str),
    Array(&'a ArrayType),
    Map(&'a MapType),
}

fn maybe_primitive(b: &BuiltinType) -> MaybePrimitive {
    MaybePrimitive::Primitive(match b {
        BuiltinType::Bool => "bool",
        BuiltinType::I8 => "sbyte",
        BuiltinType::U8 => "byte",
        BuiltinType::I16 => "short",
        BuiltinType::U16 => "ushort",
        BuiltinType::I32 => "int",
        BuiltinType::U32 => "uint",
        BuiltinType::I64 => "long",
        BuiltinType::U64 => "ulong",
        // BuiltinType::I128 => "int128", Not a supported type in csharp
        // BuiltinType::U128 => "uint128", Not a supported type in csharp
        BuiltinType::I128 => panic!("i128 not supported for csharp"),
        BuiltinType::U128 => panic!("i128 not supported for csharp"),
        BuiltinType::String => "string",
        BuiltinType::F32 => "float",
        BuiltinType::F64 => "double",
        BuiltinType::Array(ty) => return MaybePrimitive::Array(ty),
        BuiltinType::Map(m) => return MaybePrimitive::Map(m),
    })
}

fn ty_fmt<'a>(ctx: &'a GenCtx, ty: &'a AlgebraicType, namespace: &'a str) -> impl fmt::Display + 'a {
    fmt_fn(move |f| match ty {
        AlgebraicType::Sum(sum_type) => {
            // This better be an option type
            if let Some(inner_ty) = sum_type.as_option() {
                match inner_ty {
                    Builtin(b) => match b {
                        BuiltinType::Bool
                        | BuiltinType::I8
                        | BuiltinType::U8
                        | BuiltinType::I16
                        | BuiltinType::U16
                        | BuiltinType::I32
                        | BuiltinType::U32
                        | BuiltinType::I64
                        | BuiltinType::U64
                        | BuiltinType::I128
                        | BuiltinType::U128
                        | BuiltinType::F32
                        | BuiltinType::F64 => {
                            // This has to be a nullable type.
                            write!(f, "{}?", ty_fmt(ctx, inner_ty, namespace))
                        }
                        _ => {
                            write!(f, "{}", ty_fmt(ctx, inner_ty, namespace))
                        }
                    },
                    _ => {
                        write!(f, "{}", ty_fmt(ctx, inner_ty, namespace))
                    }
                }
            } else {
                unimplemented!()
            }
        }
        AlgebraicType::Product(prod) => {
            // The only type that is allowed here is the identity type. All other types should fail.
            if prod.is_identity() {
                write!(f, "SpacetimeDB.Identity")
            } else if prod.is_address() {
                write!(f, "SpacetimeDB.Address")
            } else {
                unimplemented!()
            }
        }
        AlgebraicType::Builtin(b) => match maybe_primitive(b) {
            MaybePrimitive::Primitive(p) => f.write_str(p),
            MaybePrimitive::Array(ArrayType { elem_ty }) if **elem_ty == AlgebraicType::U8 => f.write_str("byte[]"),
            MaybePrimitive::Array(ArrayType { elem_ty }) => {
                write!(
                    f,
                    "System.Collections.Generic.List<{}>",
                    ty_fmt(ctx, elem_ty, namespace)
                )
            }
            MaybePrimitive::Map(ty) => {
                write!(
                    f,
                    "System.Collections.Generic.Dictionary<{}, {}>",
                    ty_fmt(ctx, &ty.ty, namespace),
                    ty_fmt(ctx, &ty.key_ty, namespace)
                )
            }
        },
        AlgebraicType::Ref(r) => {
            let name = csharp_typename(ctx, *r);
            match &ctx.typespace.types[r.idx()] {
                AlgebraicType::Sum(sum_type) => {
                    if is_enum(sum_type) {
                        let parts: Vec<&str> = name.split('.').collect();
                        if parts.len() >= 2 {
                            let enum_namespace = parts[0];
                            let enum_name = parts[1];
                            write!(f, "{namespace}.{enum_namespace}.Types.{enum_name}")
                        } else {
                            write!(f, "{}.{}", namespace, name)
                        }
                    } else {
                        write!(f, "{}.{}", namespace, name)
                    }
                }
                _ => {
                    write!(f, "{}.{}", namespace, name)
                }
            }
        }
    })
}

// can maybe do something fancy with this in the future
fn csharp_typename(ctx: &GenCtx, typeref: AlgebraicTypeRef) -> &str {
    ctx.names[typeref.idx()].as_deref().expect("tuples should have names")
}

macro_rules! indent_scope {
    ($x:ident) => {
        let mut $x = $x.indented(1);
    };
}

pub fn is_enum(sum_type: &SumType) -> bool {
    for variant in sum_type.clone().variants {
        match variant.algebraic_type {
            AlgebraicType::Product(product) => {
                if product.elements.is_empty() {
                    continue;
                }
            }
            _ => return false,
        }
    }

    true
}

pub fn autogen_csharp_sum(
    /* will be used in future for tagged enum */ _ctx: &GenCtx,
    name: &str,
    sum_type: &SumType,
    namespace: &str,
) -> String {
    if is_enum(sum_type) {
        autogen_csharp_enum(name, sum_type, namespace)
    } else {
        unimplemented!();
    }
}

pub fn autogen_csharp_enum(name: &str, sum_type: &SumType, namespace: &str) -> String {
    let mut output = CodeIndenter::new(String::new());

    let mut sum_namespace = None;
    let mut sum_type_name = name.replace("r#", "").to_case(Case::Pascal);
    if sum_type_name.contains('.') {
        let split: Vec<&str> = sum_type_name.split('.').collect();
        if split.len() != 2 {
            panic!("Enum names cannot contain more than one namespace prefix. Example: MyNamespace.MyEnum");
        }

        sum_namespace = Some(split[0].to_string().to_case(Case::Pascal));
        sum_type_name = split[1].to_string().to_case(Case::Pascal);
    }

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE",
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "using System;").unwrap();
    if namespace != "SpacetimeDB" {
        writeln!(output, "using SpacetimeDB;").unwrap();
    }

    writeln!(output).unwrap();

    writeln!(output, "namespace {namespace}").unwrap();
    writeln!(output, "{{").unwrap();
    {
        indent_scope!(output);
        writeln!(
            output,
            "public {}",
            match sum_namespace.clone() {
                None => format!("enum {}", sum_type_name),
                Some(namespace) => format!("partial class {}", namespace),
            },
        )
        .unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            match sum_namespace {
                Some(_) => {
                    writeln!(output, "public partial class Types").unwrap();
                    writeln!(output, "{{").unwrap();
                    {
                        indent_scope!(output);
                        writeln!(output, "[SpacetimeDB.Type]").unwrap();
                        writeln!(output, "public enum {}", sum_type_name).unwrap();
                        writeln!(output, "{{").unwrap();
                        {
                            indent_scope!(output);
                            for variant in &sum_type.variants {
                                let variant_name = variant
                                    .name
                                    .as_ref()
                                    .expect("All sum variants should have names!")
                                    .replace("r#", "");
                                writeln!(output, "{},", variant_name).unwrap();
                            }
                        }
                        writeln!(output, "}}").unwrap();
                    }
                    writeln!(output, "}}").unwrap();
                }
                None => {
                    for variant in &sum_type.variants {
                        let variant_name = variant
                            .name
                            .as_ref()
                            .expect("All sum variants should have names!")
                            .replace("r#", "");
                        writeln!(output, "{},", variant_name).unwrap();
                    }
                }
            }
        }

        // End either enum or class def
        writeln!(output, "}}").unwrap();
    }
    writeln!(output, "}}").unwrap();

    output.into_inner()
}

pub fn autogen_csharp_tuple(ctx: &GenCtx, name: &str, tuple: &ProductType, namespace: &str) -> String {
    autogen_csharp_product_table_common(ctx, name, tuple, None, namespace)
}

pub fn autogen_csharp_table(ctx: &GenCtx, table: &TableDesc, namespace: &str) -> String {
    let tuple = ctx.typespace[table.data].as_product().unwrap();
    autogen_csharp_product_table_common(
        ctx,
        &table.schema.table_name,
        tuple,
        Some(
            table
                .schema
                .clone()
                .into_schema(0.into())
                .validated()
                .expect("Failed to generate table due to validation errors"),
        ),
        namespace,
    )
}

fn autogen_csharp_product_table_common(
    ctx: &GenCtx,
    name: &str,
    product_type: &ProductType,
    schema: Option<TableSchema>,
    namespace: &str,
) -> String {
    let mut output = CodeIndenter::new(String::new());

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE",
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "using System;").unwrap();
    writeln!(output, "using System.Collections.Generic;").unwrap();
    if namespace != "SpacetimeDB" {
        writeln!(output, "using SpacetimeDB;").unwrap();
    }

    writeln!(output).unwrap();

    writeln!(output, "namespace {namespace}").unwrap();
    writeln!(output, "{{").unwrap();
    {
        indent_scope!(output);
        writeln!(output, "[SpacetimeDB.Type]").unwrap();
        writeln!(output, "public partial class {name} : IDatabaseTable").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            for field in &product_type.elements {
                let field_name = field
                    .name
                    .as_ref()
                    .expect("autogen'd tuples should have field names")
                    .replace("r#", "");

                writeln!(
                    output,
                    "public {} {};",
                    ty_fmt(ctx, &field.algebraic_type, namespace),
                    field_name.to_case(Case::Pascal)
                )
                .unwrap();
            }

            writeln!(output).unwrap();

            // If this is a table, we want to generate indexes
            if let Some(schema) = &schema {
                let constraints = schema.column_constraints();
                // Declare custom index dictionaries
                for col in schema.columns() {
                    let field_name = col.col_name.replace("r#", "").to_case(Case::Pascal);
                    if !constraints[&NonEmpty::new(col.col_pos)].has_unique() {
                        continue;
                    }
                    let type_name = ty_fmt(ctx, &col.col_type, namespace);
                    let comparer = if format!("{}", type_name) == "byte[]" {
                        ", new SpacetimeDB.ByteArrayComparer()"
                    } else {
                        ""
                    };
                    writeln!(
                        output,
                        "private static Dictionary<{type_name}, {name}> {field_name}_Index = new Dictionary<{type_name}, {name}>(16{comparer});"
                    )
                        .unwrap();
                }
                writeln!(output).unwrap();
                // OnInsert method for updating indexes
                writeln!(
                    output,
                    "private static void InternalOnValueInserted(object insertedValue)"
                )
                .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(output, "var val = ({name})insertedValue;").unwrap();
                    for col in schema.columns() {
                        let field_name = col.col_name.replace("r#", "").to_case(Case::Pascal);
                        if !constraints[&NonEmpty::new(col.col_pos)].has_unique() {
                            continue;
                        }
                        writeln!(output, "{field_name}_Index[val.{field_name}] = val;").unwrap();
                    }
                }
                writeln!(output, "}}").unwrap();
                writeln!(output).unwrap();
                // OnDelete method for updating indexes
                writeln!(
                    output,
                    "private static void InternalOnValueDeleted(object deletedValue)"
                )
                .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(output, "var val = ({name})deletedValue;").unwrap();
                    for col in schema.columns() {
                        let field_name = col.col_name.replace("r#", "").to_case(Case::Pascal);
                        if !constraints[&NonEmpty::new(col.col_pos)].has_unique() {
                            continue;
                        }
                        writeln!(output, "{field_name}_Index.Remove(val.{field_name});").unwrap();
                    }
                }
                writeln!(output, "}}").unwrap();
                writeln!(output).unwrap();
            } // End indexes

            // If this is a table, we want to include functions for accessing the table data
            if let Some(column_attrs) = &schema {
                // Insert the funcs for accessing this struct
                let has_primary_key = autogen_csharp_access_funcs_for_struct(
                    &mut output,
                    name,
                    product_type,
                    name,
                    column_attrs,
                    ctx,
                    namespace,
                );

                writeln!(output).unwrap();

                writeln!(
                    output,
                    "public delegate void InsertEventHandler({name} insertedValue, {namespace}.ReducerEvent dbEvent);"
                )
                .unwrap();
                if has_primary_key {
                    writeln!(output, "public delegate void UpdateEventHandler({name} oldValue, {name} newValue, {namespace}.ReducerEvent dbEvent);").unwrap();
                }
                writeln!(
                    output,
                    "public delegate void DeleteEventHandler({name} deletedValue, {namespace}.ReducerEvent dbEvent);"
                )
                .unwrap();
                writeln!(output, "public delegate void RowUpdateEventHandler(SpacetimeDBClient.TableOp op, {name} oldValue, {name} newValue, {namespace}.ReducerEvent dbEvent);").unwrap();
                writeln!(output, "public static event InsertEventHandler OnInsert;").unwrap();
                if has_primary_key {
                    writeln!(output, "public static event UpdateEventHandler OnUpdate;").unwrap();
                }
                writeln!(output, "public static event DeleteEventHandler OnBeforeDelete;").unwrap();
                writeln!(output, "public static event DeleteEventHandler OnDelete;").unwrap();

                writeln!(output, "public static event RowUpdateEventHandler OnRowUpdate;").unwrap();

                writeln!(output).unwrap();

                writeln!(
                    output,
                    "public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)"
                )
                .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(
                        output,
                        "OnInsert?.Invoke(({name})newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);"
                    )
                    .unwrap();
                }
                writeln!(output, "}}").unwrap();
                writeln!(output).unwrap();

                if has_primary_key {
                    writeln!(
                        output,
                        "public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)"
                    )
                    .unwrap();
                    writeln!(output, "{{").unwrap();
                    {
                        indent_scope!(output);
                        writeln!(
                            output,
                            "OnUpdate?.Invoke(({name})oldValue,({name})newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);"
                        )
                            .unwrap();
                    }
                    writeln!(output, "}}").unwrap();
                    writeln!(output).unwrap();
                }

                writeln!(
                    output,
                    "public static void OnBeforeDeleteEvent(object oldValue, ClientApi.Event dbEvent)"
                )
                .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(
                        output,
                        "OnBeforeDelete?.Invoke(({name})oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);"
                    )
                    .unwrap();
                }
                writeln!(output, "}}").unwrap();
                writeln!(output).unwrap();

                writeln!(
                    output,
                    "public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)"
                )
                .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(
                        output,
                        "OnDelete?.Invoke(({name})oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);"
                    )
                    .unwrap();
                }
                writeln!(output, "}}").unwrap();
                writeln!(output).unwrap();

                writeln!(
                    output,
                    "public static void OnRowUpdateEvent(SpacetimeDBClient.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)"
                )
                    .unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(
                        output,
                        "OnRowUpdate?.Invoke(op, ({name})oldValue,({name})newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);"
                    )
                        .unwrap();
                }
                writeln!(output, "}}").unwrap();
            }
        }
        writeln!(output, "}}").unwrap();
    }
    writeln!(output, "}}").unwrap();

    output.into_inner()
}

fn indented_block<R>(output: &mut CodeIndenter<String>, f: impl FnOnce(&mut CodeIndenter<String>) -> R) -> R {
    writeln!(output, "{{").unwrap();
    let res = f(&mut output.indented(1));
    writeln!(output, "}}").unwrap();
    res
}

fn autogen_csharp_access_funcs_for_struct(
    output: &mut CodeIndenter<String>,
    struct_name_pascal_case: &str,
    product_type: &ProductType,
    table_name: &str,
    schema: &TableSchema,
    ctx: &GenCtx,
    namespace: &str,
) -> bool {
    let primary_col_idx = schema.pk();

    writeln!(
        output,
        "public static System.Collections.Generic.IEnumerable<{struct_name_pascal_case}> Iter()"
    )
    .unwrap();
    indented_block(output, |output| {
        writeln!(
            output,
            "return SpacetimeDBClient.clientDB.GetObjects<{struct_name_pascal_case}>();"
        )
        .unwrap();
    });

    writeln!(output, "public static int Count()").unwrap();
    indented_block(output, |output| {
        writeln!(
            output,
            "return SpacetimeDBClient.clientDB.Count<{struct_name_pascal_case}>();",
        )
        .unwrap();
    });

    let constraints = schema.column_constraints();
    for col in schema.columns() {
        let is_unique = constraints[&NonEmpty::new(col.col_pos)].has_unique();

        let col_i: usize = col.col_pos.into();

        let field = &product_type.elements[col_i];
        let field_name = field.name.as_ref().expect("autogen'd tuples should have field names");
        let field_type = &field.algebraic_type;
        let csharp_field_type = ty_fmt(ctx, field_type, namespace);
        let csharp_field_name_pascal = field_name.replace("r#", "").to_case(Case::Pascal);

        let filter_return_type = fmt_fn(|f| {
            if is_unique {
                f.write_str(struct_name_pascal_case)
            } else {
                write!(f, "System.Collections.Generic.IEnumerable<{}>", struct_name_pascal_case)
            }
        });

        writeln!(
            output,
            "public static {filter_return_type} FilterBy{}({} value)",
            csharp_field_name_pascal, csharp_field_type
        )
        .unwrap();

        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            if is_unique {
                writeln!(
                    output,
                    "{csharp_field_name_pascal}_Index.TryGetValue(value, out var r);"
                )
                .unwrap();
                writeln!(output, "return r;").unwrap();
            } else {
                write!(
                    output,
                    "return Iter().Where(x => x.{csharp_field_name_pascal} == value)"
                )
                .unwrap();

                if is_unique {
                    write!(output, ".SingleOrDefault()").unwrap();
                }

                writeln!(output, ";").unwrap();
            }
        }
        // End Func
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();
    }

    if let Some(primary_col_index) = primary_col_idx {
        writeln!(output, "public static object GetPrimaryKeyValue(object v)").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            writeln!(
                output,
                "return (({struct_name_pascal_case})v).{col_name_pascal_case};",
                col_name_pascal_case = primary_col_index.col_name.replace("r#", "").to_case(Case::Pascal)
            )
            .unwrap();
        }
        writeln!(output, "}}").unwrap();
    }

    primary_col_idx.is_some()
}

pub fn autogen_csharp_reducer(ctx: &GenCtx, reducer: &ReducerDef, namespace: &str) -> String {
    let func_name = &*reducer.name;
    // let reducer_pascal_name = func_name.to_case(Case::Pascal);
    let use_namespace = true;
    let func_name_pascal_case = func_name.to_case(Case::Pascal);

    let mut output = CodeIndenter::new(String::new());

    let mut func_params: String = String::new();
    let mut struct_fields: String = String::new();
    let mut field_inits: String = String::new();

    for (arg_i, arg) in reducer.args.iter().enumerate() {
        let name = arg
            .name
            .as_deref()
            .unwrap_or_else(|| panic!("reducer args should have names: {}", func_name));
        let arg_name = name.to_case(Case::Camel);
        let field_name = name.to_case(Case::Pascal);
        let arg_type_str = ty_fmt(ctx, &arg.algebraic_type, namespace);

        if arg_i != 0 {
            func_params.push_str(", ");
            field_inits.push_str(", ");
        }
        write!(func_params, "{arg_type_str} {arg_name}").unwrap();
        write!(struct_fields, "public {arg_type_str} {field_name};").unwrap();
        write!(field_inits, "{field_name} = {arg_name}").unwrap();
    }

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE"
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "using System;").unwrap();
    writeln!(output, "using ClientApi;").unwrap();
    writeln!(output, "using CommunityToolkit.HighPerformance;").unwrap();
    if namespace != "SpacetimeDB" {
        writeln!(output, "using SpacetimeDB;").unwrap();
    }

    writeln!(output).unwrap();

    if use_namespace {
        writeln!(output, "namespace {}", namespace).unwrap();
        writeln!(output, "{{").unwrap();
        output.indent(1);
    }

    writeln!(output, "public static partial class Reducer").unwrap();
    writeln!(output, "{{").unwrap();

    {
        indent_scope!(output);

        let delegate_separator = if !reducer.args.is_empty() { ", " } else { "" };
        writeln!(
            output,
            "public delegate void {func_name_pascal_case}Handler(ReducerEvent reducerEvent{delegate_separator}{func_params});"
        )
        .unwrap();
        writeln!(
            output,
            "public static event {func_name_pascal_case}Handler On{func_name_pascal_case}Event;"
        )
        .unwrap();

        writeln!(output).unwrap();

        writeln!(output, "public static void {func_name_pascal_case}({func_params})").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            writeln!(
                output,
                "SpacetimeDBClient.instance.InternalCallReducer(\"{reducer_name}\", new {func_name_pascal_case}ArgsStruct {{ {field_inits} }});",
                reducer_name = reducer.name
            )
            .unwrap();
        }
        // Closing brace for reducer
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "[ReducerCallback(FunctionName = \"{func_name}\")]").unwrap();
        writeln!(
            output,
            "public static bool On{func_name_pascal_case}(ClientApi.Event dbEvent)"
        )
        .unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            writeln!(output, "if(On{func_name_pascal_case}Event != null)").unwrap();
            writeln!(output, "{{").unwrap();
            {
                indent_scope!(output);
                writeln!(
                    output,
                    "var args = ((ReducerEvent)dbEvent.FunctionCall.CallInfo).{func_name_pascal_case}Args;"
                )
                .unwrap();
                writeln!(
                    output,
                    "On{func_name_pascal_case}Event((ReducerEvent)dbEvent.FunctionCall.CallInfo"
                )
                .unwrap();
                // Write out arguments one per line
                {
                    indent_scope!(output);
                    for (i, arg) in reducer.args.iter().enumerate() {
                        let arg_name = arg
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg_{}", i))
                            .to_case(Case::Pascal);
                        let arg_type_str = ty_fmt(ctx, &arg.algebraic_type, namespace);
                        writeln!(output, ",({arg_type_str})args.{arg_name}").unwrap();
                    }
                }
                writeln!(output, ");").unwrap();
                writeln!(output, "return true;").unwrap();
            }
            // Closing brace for if event is registered
            writeln!(output, "}}").unwrap();
            writeln!(output, "return false;").unwrap();
        }
        // Closing brace for Event parsing function
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "[DeserializeEvent(FunctionName = \"{func_name}\")]").unwrap();
        writeln!(
            output,
            "public static void {func_name_pascal_case}DeserializeEventArgs(ClientApi.Event dbEvent)"
        )
        .unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);

            writeln!(output, "dbEvent.FunctionCall.CallInfo = new ReducerEvent(").unwrap();
            {
                indent_scope!(output);

                writeln!(output, "ReducerType.{func_name_pascal_case},").unwrap();
                writeln!(output, "\"{func_name}\",").unwrap();
                writeln!(output, "dbEvent.Timestamp,").unwrap();
                writeln!(output, "Identity.From(dbEvent.CallerIdentity.ToByteArray()),").unwrap();
                writeln!(output, "Address.From(dbEvent.CallerAddress.ToByteArray()),").unwrap();
                writeln!(output, "dbEvent.Message,").unwrap();
                writeln!(output, "dbEvent.Status,").unwrap();
                writeln!(
                    output,
                    "BSATNHelpers.FromProtoBytes<{func_name_pascal_case}ArgsStruct>(dbEvent.FunctionCall.ArgBytes)"
                )
                .unwrap();
            }
            writeln!(output, ");").unwrap();
        }

        // Closing brace for Event parsing function
        writeln!(output, "}}").unwrap();
    }
    // Closing brace for class
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    //Args struct
    writeln!(output, "[SpacetimeDB.Type]").unwrap();
    writeln!(output, "public partial class {func_name_pascal_case}ArgsStruct {{").unwrap();
    writeln!(output, "{struct_fields}").unwrap();
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    if use_namespace {
        output.dedent(1);
        writeln!(output, "}}").unwrap();
    }

    output.into_inner()
}

pub fn autogen_csharp_globals(items: &[GenItem], namespace: &str) -> Vec<Vec<(String, String)>> {
    let reducers: Vec<&ReducerDef> = items
        .iter()
        .filter_map(|i| {
            if let GenItem::Reducer(reducer) = i {
                Some(reducer)
            } else {
                None
            }
        })
        .collect();
    let reducer_names: Vec<String> = reducers
        .iter()
        .map(|reducer| reducer.name.to_case(Case::Pascal))
        .collect();

    let use_namespace = true;
    let mut output = CodeIndenter::new(String::new());

    writeln!(
        output,
        "// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE"
    )
    .unwrap();
    writeln!(output, "// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "using System;").unwrap();
    writeln!(output, "using ClientApi;").unwrap();
    if namespace != "SpacetimeDB" {
        writeln!(output, "using SpacetimeDB;").unwrap();
    }

    writeln!(output).unwrap();

    if use_namespace {
        writeln!(output, "namespace {}", namespace).unwrap();
        writeln!(output, "{{").unwrap();
        output.indent(1);
    }

    writeln!(output, "public enum ReducerType").unwrap();
    writeln!(output, "{{").unwrap();
    {
        indent_scope!(output);
        writeln!(output, "None,").unwrap();
        for reducer in reducer_names {
            writeln!(output, "{reducer},").unwrap();
        }
    }
    // Closing brace for ReducerType
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "public partial class ReducerEvent : ReducerEventBase").unwrap();
    writeln!(output, "{{").unwrap();
    {
        indent_scope!(output);
        writeln!(output, "public ReducerType Reducer {{ get; private set; }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "public ReducerEvent(ReducerType reducer, string reducerName, ulong timestamp, SpacetimeDB.Identity identity, SpacetimeDB.Address? callerAddress, string errMessage, ClientApi.Event.Types.Status status, object args)").unwrap();
        {
            indent_scope!(output);
            writeln!(
                output,
                ": base(reducerName, timestamp, identity, callerAddress, errMessage, status, args)"
            )
            .unwrap();
        }
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            writeln!(output, "Reducer = reducer;").unwrap();
        }
        // Closing brace for ctor
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();
        // Properties for reducer args
        for reducer in &reducers {
            let reducer_name = reducer.name.to_case(Case::Pascal);
            writeln!(output, "public {reducer_name}ArgsStruct {reducer_name}Args").unwrap();
            writeln!(output, "{{").unwrap();
            {
                indent_scope!(output);
                writeln!(output, "get").unwrap();
                writeln!(output, "{{").unwrap();
                {
                    indent_scope!(output);
                    writeln!(output, "if (Reducer != ReducerType.{reducer_name}) throw new SpacetimeDB.ReducerMismatchException(Reducer.ToString(), \"{reducer_name}\");").unwrap();
                    writeln!(output, "return ({reducer_name}ArgsStruct)Args;").unwrap();
                }
                // Closing brace for struct ReducerArgs
                writeln!(output, "}}").unwrap();
            }
            // Closing brace for struct ReducerArgs
            writeln!(output, "}}").unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "public object[] GetArgsAsObjectArray()").unwrap();
        writeln!(output, "{{").unwrap();
        {
            indent_scope!(output);
            writeln!(output, "switch (Reducer)").unwrap();
            writeln!(output, "{{").unwrap();
            {
                indent_scope!(output);
                for reducer in &reducers {
                    let reducer_name = reducer.name.to_case(Case::Pascal);
                    writeln!(output, "case ReducerType.{reducer_name}:").unwrap();
                    writeln!(output, "{{").unwrap();
                    {
                        indent_scope!(output);
                        writeln!(output, "var args = {reducer_name}Args;").unwrap();
                        writeln!(output, "return new object[] {{").unwrap();
                        {
                            indent_scope!(output);
                            for (i, arg) in reducer.args.iter().enumerate() {
                                let arg_name = arg
                                    .name
                                    .clone()
                                    .unwrap_or_else(|| format!("arg_{}", i))
                                    .to_case(Case::Pascal);
                                writeln!(output, "args.{arg_name},").unwrap();
                            }
                        }
                        writeln!(output, "}};").unwrap();
                    }
                    // Closing brace for switch
                    writeln!(output, "}}").unwrap();
                }
                writeln!(output, "default: throw new System.Exception($\"Unhandled reducer case: {{Reducer}}. Please run SpacetimeDB code generator\");").unwrap();
            }
            // Closing brace for switch
            writeln!(output, "}}").unwrap();
        }
        // Closing brace for ctor
        writeln!(output, "}}").unwrap();
    }
    // Closing brace for ReducerEvent
    writeln!(output, "}}").unwrap();

    if use_namespace {
        output.dedent(1);
        writeln!(output, "}}").unwrap();
    }

    vec![vec![("ReducerEvent.cs".to_string(), output.into_inner())]]
}
