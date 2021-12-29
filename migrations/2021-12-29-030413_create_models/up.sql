-- Your SQL goes here

/* HOMEPAGE testimonials  */
CREATE TABLE IF NOT EXISTS testimonials (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    testimonial TEXT NOT NULL,
    workplace TEXT NOT NULL,
    website TEXT NOT NULL
);

/* Project used in the portfolio section in the ABOUT US page */
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    site TEXT NOT NULL,
    description TEXT NOT NULL
);

/* Free Software project shown in the Services page of the site */
CREATE TABLE IF NOT EXISTS fsprojects (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    github_addr TEXT NOT NULL,
    support_addr TEXT NOT NULL,
    proj_license TEXT NOT NULL,
    license_link TEXT NOT NULL
);

/* License text shown in the LICENSES page */
CREATE TABLE IF NOT EXISTS licenses (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    verbatim TEXT NOT NULL,
    license_link TEXT NOT NULL
);

/* Hall Of Fame project */
CREATE TABLE IF NOT EXISTS hofs (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

/* Team member table to show on the team page */
CREATE TABLE IF NOT EXISTS teammembers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    position TEXT NOT NULL,
    is_collab BOOLEAN NOT NULL
);

/* Privacy policy table */
CREATE TABLE IF NOT EXISTS privacypolicies (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    changelog TEXT NOT NULL,
    text TEXT NOT NULL
);

/* Terms of Service Table */
CREATE TABLE IF NOT EXISTS termsofservices (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    changelog TEXT NOT NULL,
    text TEXT NOT NULL
);

/* Blog Post table */
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY NOT NULL,
    published BOOLEAN NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    content TEXT NOT NULL
);

/* Product table */
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    category TEXT NOT NULL,
    apptype TEXT NOT NULL,
    short_description TEXT NOT NULL,
    description TEXT NOT NULL,
    stripe_link TEXT NOT NULL,
    available BOOLEAN NOT NULL
);
