table! {
    fsprojects (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        github_addr -> Text,
        support_addr -> Text,
        proj_license -> Text,
        license_link -> Text,
    }
}

table! {
    hofs (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    licenses (id) {
        id -> Integer,
        name -> Text,
        verbatim -> Text,
        license_link -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        published -> Bool,
        title -> Text,
        description -> Text,
        category -> Text,
        content -> Text,
    }
}

table! {
    privacypolicies (id) {
        id -> Integer,
        title -> Text,
        changelog -> Text,
        text -> Text,
    }
}

table! {
    products (id) {
        id -> Integer,
        name -> Text,
        price -> Float,
        category -> Text,
        apptype -> Text,
        short_description -> Text,
        description -> Text,
        stripe_link -> Text,
        available -> Bool,
    }
}

table! {
    projects (id) {
        id -> Integer,
        title -> Text,
        site -> Text,
        description -> Text,
    }
}

table! {
    teammembers (id) {
        id -> Integer,
        name -> Text,
        position -> Text,
        is_collab -> Bool,
    }
}

table! {
    termsofservices (id) {
        id -> Integer,
        title -> Text,
        changelog -> Text,
        text -> Text,
    }
}

table! {
    testimonials (id) {
        id -> Integer,
        name -> Text,
        testimonial -> Text,
        workplace -> Text,
        website -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    fsprojects,
    hofs,
    licenses,
    posts,
    privacypolicies,
    products,
    projects,
    teammembers,
    termsofservices,
    testimonials,
);
