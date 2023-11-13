// @generated automatically by Diesel CLI.



diesel::table! {
    persons (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
