diesel::table! {
    trees (account_hash) {
        account_hash -> VarChar,
        account_id -> VarChar,
        merkle_root -> VarChar,
        proof_file_id -> VarChar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    messages (hash) {
        hash -> VarChar,
        parent_account_hash -> VarChar,
        ornament_id -> Integer,
        nickname -> VarChar,
        proof_file_id -> VarChar,
        created_at -> Timestamp,
    }
}