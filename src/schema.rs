table! {
    fsprojects (id) {
        id -> Text,
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
        id -> Text,
        name -> Text,
    }
}

table! {
    licenses (id) {
        id -> Text,
        name -> Text,
        verbatim -> Text,
        license_link -> Text,
    }
}

table! {
    posts (id) {
        id -> Text,
        published -> Bool,
        title -> Text,
        description -> Text,
        category -> Text,
        content -> Text,
    }
}

table! {
    privacypolicies (id) {
        id -> Text,
        title -> Text,
        changelog -> Text,
        text -> Text,
    }
}

table! {
    products (id) {
        id -> Text,
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
        id -> Text,
        title -> Text,
        site -> Text,
        description -> Text,
    }
}

table! {
    teammembers (id) {
        id -> Text,
        name -> Text,
        position -> Text,
        is_collab -> Bool,
    }
}

table! {
    termsofservices (id) {
        id -> Text,
        title -> Text,
        changelog -> Text,
        text -> Text,
    }
}

table! {
    testimonials (id) {
        id -> Text,
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
