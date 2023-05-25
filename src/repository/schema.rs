// @generated automatically by Diesel CLI.

diesel::table! {
    mice (id) {
        id -> Varchar,
        brand -> Varchar,
        model -> Nullable<Varchar>,
        width -> Array<Nullable<Float8>>,
        height -> Array<Nullable<Float8>>,
        length -> Float8,
        weight -> Float8,
        shape -> Text,
        wireless -> Bool,
        sensor -> Nullable<Varchar>,
        mcu -> Nullable<Varchar>,
        dpi -> Nullable<Varchar>,
        polling_rate -> Nullable<Varchar>,
        switches -> Nullable<Varchar>,
        mouse_wheel_encoder -> Nullable<Varchar>,
        material -> Text,
        launch_date -> Nullable<Date>,
    }
}
