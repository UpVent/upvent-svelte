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
    projects (id) {
        id -> Text,
        title -> Text,
        site -> Text,
        description -> Text,
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
    projects,
    testimonials,
);
