//! flash.geom.Point

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{ExecutionReason, FunctionObject};
use crate::avm1::property_decl::{define_properties_on, Declaration};
use crate::avm1::{Object, Value};
use crate::string::{AvmString, StringContext};

use ruffle_macros::istr;

const PROTO_DECLS: &[Declaration] = declare_properties! {
    "toString" => method(to_string);
    "clone" => method(clone);
    "equals" => method(equals);
    "add" => method(add);
    "subtract" => method(subtract);
    "normalize" => method(normalize);
    "offset" => method(offset);
    "length" => property(length; READ_ONLY);
};

const OBJECT_DECLS: &[Declaration] = declare_properties! {
    "distance" => method(distance);
    "polar" => method(polar);
    "interpolate" => method(interpolate);
};

pub fn point_to_object<'gc>(
    point: (f64, f64),
    activation: &mut Activation<'_, 'gc>,
) -> Result<Value<'gc>, Error<'gc>> {
    let args = [point.0.into(), point.1.into()];
    construct_new_point(&args, activation)
}

pub fn construct_new_point<'gc>(
    args: &[Value<'gc>],
    activation: &mut Activation<'_, 'gc>,
) -> Result<Value<'gc>, Error<'gc>> {
    let constructor = activation.context.avm1.prototypes().point_constructor;
    let object = constructor.construct(activation, args)?;
    Ok(object)
}

pub fn value_to_point<'gc>(
    value: Value<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<(f64, f64), Error<'gc>> {
    let x = value
        .coerce_to_object(activation)
        .get(istr!("x"), activation)?
        .coerce_to_f64(activation)?;
    let y = value
        .coerce_to_object(activation)
        .get(istr!("y"), activation)?
        .coerce_to_f64(activation)?;
    Ok((x, y))
}

pub fn object_to_point<'gc>(
    object: Object<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<(f64, f64), Error<'gc>> {
    let x = object
        .get(istr!("x"), activation)?
        .coerce_to_f64(activation)?;
    let y = object
        .get(istr!("y"), activation)?
        .coerce_to_f64(activation)?;
    Ok((x, y))
}

fn constructor<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.is_empty() {
        this.set(istr!("x"), 0.into(), activation)?;
        this.set(istr!("y"), 0.into(), activation)?;
    } else {
        this.set(
            istr!("x"),
            args.get(0).unwrap_or(&Value::Undefined).to_owned(),
            activation,
        )?;
        this.set(
            istr!("y"),
            args.get(1).unwrap_or(&Value::Undefined).to_owned(),
            activation,
        )?;
    }

    Ok(Value::Undefined)
}

fn clone<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let args = [
        this.get(istr!("x"), activation)?,
        this.get(istr!("y"), activation)?,
    ];
    let constructor = activation.context.avm1.prototypes().point_constructor;
    let cloned = constructor.construct(activation, &args)?;

    Ok(cloned)
}

fn equals<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(other) = args.get(0) {
        let this_x = this.get(istr!("x"), activation)?;
        let this_y = this.get(istr!("y"), activation)?;
        let other = other.coerce_to_object(activation);
        let other_x = other.get(istr!("x"), activation)?;
        let other_y = other.get(istr!("y"), activation)?;
        return Ok((this_x == other_x && this_y == other_y).into());
    }

    Ok(false.into())
}

fn add<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this_x = this
        .get(istr!("x"), activation)?
        .coerce_to_f64(activation)?;
    let this_y = this
        .get(istr!("y"), activation)?
        .coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x + other.0, this_y + other.1), activation)?;
    Ok(object)
}

fn subtract<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this_x = this
        .get(istr!("x"), activation)?
        .coerce_to_f64(activation)?;
    let this_y = this
        .get(istr!("y"), activation)?
        .coerce_to_f64(activation)?;
    let other = value_to_point(
        args.get(0).unwrap_or(&Value::Undefined).to_owned(),
        activation,
    )?;
    let object = point_to_object((this_x - other.0, this_y - other.1), activation)?;
    Ok(object)
}

fn distance<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.len() < 2 {
        return Ok(f64::NAN.into());
    }

    let a = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_object(activation);
    let b = args.get(1).unwrap_or(&Value::Undefined);
    let delta = a.call_method(
        istr!("subtract"),
        &[b.to_owned()],
        activation,
        ExecutionReason::FunctionCall,
    )?;
    delta
        .coerce_to_object(activation)
        .get(istr!("length"), activation)
}

fn polar<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let length = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let angle = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let point = point_to_object((length * angle.cos(), length * angle.sin()), activation)?;
    Ok(point)
}

fn interpolate<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if args.len() < 3 {
        return point_to_object((f64::NAN, f64::NAN), activation);
    }

    let a = value_to_point(args.get(0).unwrap().to_owned(), activation)?;
    let b = value_to_point(args.get(1).unwrap().to_owned(), activation)?;
    let f = args.get(2).unwrap().coerce_to_f64(activation)?;
    let result = (b.0 - (b.0 - a.0) * f, b.1 - (b.1 - a.1) * f);
    point_to_object(result, activation)
}

fn to_string<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let x = this.get(istr!("x"), activation)?;
    let y = this.get(istr!("y"), activation)?;

    Ok(AvmString::new_utf8(
        activation.gc(),
        format!(
            "(x={}, y={})",
            x.coerce_to_string(activation)?,
            y.coerce_to_string(activation)?
        ),
    )
    .into())
}

fn length<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let point = value_to_point(this.into(), activation)?;
    let length = (point.0 * point.0 + point.1 * point.1).sqrt();
    Ok(length.into())
}

fn normalize<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let current_length = this
        .get(istr!("length"), activation)?
        .coerce_to_f64(activation)?;

    if current_length.is_finite() {
        let point = object_to_point(this, activation)?;
        let new_length = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .coerce_to_f64(activation)?;
        let (x, y) = if current_length == 0.0 {
            (point.0 * new_length, point.1 * new_length)
        } else {
            (
                point.0 / current_length * new_length,
                point.1 / current_length * new_length,
            )
        };

        this.set(istr!("x"), x.into(), activation)?;
        this.set(istr!("y"), y.into(), activation)?;
    }

    Ok(Value::Undefined)
}

fn offset<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let point = value_to_point(this.into(), activation)?;
    let dx = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;
    let dy = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_f64(activation)?;

    this.set(istr!("x"), (point.0 + dx).into(), activation)?;
    this.set(istr!("y"), (point.1 + dy).into(), activation)?;

    Ok(Value::Undefined)
}

pub fn create_point_object<'gc>(
    context: &mut StringContext<'gc>,
    point_proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let point = FunctionObject::native(context, constructor, fn_proto, point_proto);
    define_properties_on(OBJECT_DECLS, context, point, fn_proto);
    point
}

pub fn create_proto<'gc>(
    context: &mut StringContext<'gc>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let object = Object::new(context, Some(proto));
    define_properties_on(PROTO_DECLS, context, object, fn_proto);
    object
}
