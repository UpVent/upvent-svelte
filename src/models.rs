use rocket::serde::Deserialize;
use rocket::serde::Serialize;

use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/* Testimonial Model related imports */
use crate::schema::testimonials;
use crate::schema::testimonials::dsl::testimonials as all_testimonials;

/* Project Model related imports */
use crate::schema::projects;
use crate::schema::projects::dsl::projects as all_projects;

/* Free Software Model related imports */
use crate::schema::fsprojects;
use crate::schema::fsprojects::dsl::fsprojects as all_fsprojects;

/* License Model related imports */
use crate::schema::licenses;
use crate::schema::licenses::dsl::licenses as all_licenses;

/* Hall Of Fame Model related imports */
use crate::schema::hofs;
use crate::schema::hofs::dsl::hofs as all_hofs;

/* Team Member Model related imports */
use crate::schema::teammembers;
use crate::schema::teammembers::dsl::teammembers as all_teammembers;

/* Privacy Policy Model related imports */
use crate::schema::privacypolicies;
use crate::schema::privacypolicies::dsl::privacypolicies as all_privacypolicies;

/* Terms Of Service Model related imports */
use crate::schema::termsofservices;
use crate::schema::termsofservices::dsl::termsofservices as all_termsofservices;

/* Blog Post Model related imports */
use crate::schema::posts;
use crate::schema::posts::dsl::posts as all_posts;

/* Marketcloud product related imports */
use crate::schema::products;
use crate::schema::products::dsl::products as all_products;

/// Stores a single instance of a testimonial made by any UpVent client.
/// (Home)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Testimonial {
    pub id: i32,
    pub name: String,
    pub testimonial: String,
    pub workplace: String,
    pub website: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "testimonials"]
pub struct NewTestimonial {
    pub name: String,
    pub testimonial: String,
    pub workplace: String,
    pub website: String,
}

impl Testimonial {
    /// Returns a vector consisting of a single Testimonial in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The testimonial ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first testimonial saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let first = Testimonial::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .find(id)
            .load::<Testimonial>(conn)
            .expect("Error loading testimonial")
    }

    /// Returns a vector of all Testimonials saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first testimonial saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let test_list = Testimonial::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .order(testimonials::id.desc())
            .load::<Testimonial>(conn)
            .expect("Error loading testimonials")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, test: NewTestimonial) -> bool {
        use crate::schema::testimonials::dsl::{
            name as n, testimonial as t, website as ws, workplace as w,
        };

        let NewTestimonial {
            name,
            testimonial,
            workplace,
            website,
        } = test;

        diesel::update(all_testimonials.find(id))
            .set((
                n.eq(name),
                t.eq(testimonial),
                w.eq(workplace),
                ws.eq(website),
            ))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * test: A `NewTestimonial` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the Testimonials table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let testimonial = models::NewTestimonial {
    ///    name: String::from("VentGrey"),
    ///    testimonial: String::from("This testimonial text is a test. The cake is a lie!"),
    ///    workplace: String::from("UpVent Technologies"),
    ///    website: String::from("https://upvent.codes/"),
    /// };
    ///
    /// if models::Testimonial::insert(testimonial, &conn) {
    ///    println!("Testimonial inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the testimonial!");
    /// }
    /// ```
    ///
    pub fn insert(test: NewTestimonial, conn: &SqliteConnection) -> bool {
        diesel::insert_into(testimonials::table)
            .values(&test)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if Testimonial::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_testimonials.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<Testimonial> {
        all_testimonials
            .filter(testimonials::name.eq(name))
            .load::<Testimonial>(conn)
            .expect("Error loading testimonials by name")
    }
}

/// Stores a single instance of a project used in the portfolio section in
/// this site.
/// (About Us)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Project {
    id: i32,
    title: String,
    site: String,
    description: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "projects"]
pub struct NewProject {
    title: String,
    site: String,
    description: String,
}

impl Project {
    /// Returns a vector consisting of a single Project in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The project ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first project saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let first = Project::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Project> {
        all_projects
            .find(id)
            .load::<Project>(conn)
            .expect("Error loading testimonial")
    }

    /// Returns a vector of all Projects saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first project saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let test_list = Project::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<Project> {
        all_projects
            .order(projects::id.desc())
            .load::<Project>(conn)
            .expect("Error loading testimonials")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, proj: NewProject) -> bool {
        use crate::schema::projects::dsl::{description as d, site as s, title as t};

        let NewProject {
            title,
            site,
            description,
        } = proj;

        diesel::update(all_projects.find(id))
            .set((t.eq(title), d.eq(description), s.eq(site)))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * test: A `NewProject` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the Projects table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let project = models::NewProject {
    ///    title: String::from("VentGrey"),
    ///    site: String::from("This testimonial text is a test. The cake is a lie!"),
    ///    description: String::from("UpVent Technologies"),
    /// };
    ///
    /// if models::Project::insert(project, &conn) {
    ///    println!("Project inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the project!");
    /// }
    /// ```
    ///
    pub fn insert(proj: NewProject, conn: &SqliteConnection) -> bool {
        diesel::insert_into(projects::table)
            .values(&proj)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if Project::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_projects.find(id)).execute(conn).is_ok()
    }

    /// Documentation pending
    pub fn all_by_title(title: String, conn: &SqliteConnection) -> Vec<Project> {
        all_projects
            .filter(projects::title.eq(title))
            .load::<Project>(conn)
            .expect("Error loading projects by title")
    }
}

/// Stores a single Free Software Project shown in the "services" page
/// of this site.
/// (Services)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FSProject {
    id: i32,
    title: String,
    description: String,
    github_addr: String,
    support_addr: String,
    proj_license: String,
    license_link: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "fsprojects"]
pub struct NewFSProject {
    title: String,
    description: String,
    github_addr: String,
    support_addr: String,
    proj_license: String,
    license_link: String,
}

impl FSProject {
    /// Returns a vector consisting of a single Free Software Project in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The freee software project ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first free software project saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let first = FSProject::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<FSProject> {
        all_fsprojects
            .find(id)
            .load::<FSProject>(conn)
            .expect("Error loading free software project")
    }

    /// Returns a vector of all Free Software Projects saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first free software project saved in the database
    ///
    /// fn establish_connection() -> SqliteConnection {
    /// dotenv().ok();
    ///
    ///     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ///         SqliteConnection::establish(&database_url)
    ///         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    /// }
    ///
    /// let sqlite_connection = &mut establish_connection();
    /// let test_list = FSProject::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<FSProject> {
        all_fsprojects
            .order(fsprojects::id.desc())
            .load::<FSProject>(conn)
            .expect("Error loading free software projects")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, fsproj: NewFSProject) -> bool {
        use crate::schema::fsprojects::dsl::{
            description as d, github_addr as ga, license_link as ll, proj_license as pl,
            support_addr as sa, title as t,
        };

        let NewFSProject {
            title,
            description,
            github_addr,
            support_addr,
            proj_license,
            license_link,
        } = fsproj;

        diesel::update(all_fsprojects.find(id))
            .set((
                t.eq(title),
                d.eq(description),
                ga.eq(github_addr),
                sa.eq(support_addr),
                pl.eq(proj_license),
                ll.eq(license_link),
            ))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * fsproj: A `NewFSProject` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the FSProject table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let fsproject = models::NewFSProject {
    ///    title: String::from("Orchid"),
    ///    description: String::from("Excecute common server maintenance tasks from this script"),
    ///    github_addr: String::from("https://github.com/upvent/orchid"),
    ///    support_addr: String::from("https://github.com/upvent/orchid/issues"),
    ///    proj_license: String::from("GPL-2"),
    ///    license_link: String::from("https://gnu.org/"),
    /// };
    ///
    /// if models::FSProject::insert(fsproject, &conn) {
    ///    println!("Free Software Project inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the free software project!");
    /// }
    /// ```
    ///
    pub fn insert(fsproj: NewFSProject, conn: &SqliteConnection) -> bool {
        diesel::insert_into(fsprojects::table)
            .values(&fsproj)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if FSProject::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_fsprojects.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_title(title: String, conn: &SqliteConnection) -> Vec<FSProject> {
        all_fsprojects
            .filter(fsprojects::title.eq(title))
            .load::<FSProject>(conn)
            .expect("Error loading free software projects by title")
    }
}

/// Stores a single License text shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct License {
    id: i32,
    name: String,
    verbatim: String,
    license_link: String,
}

impl License {}

/// Stores a single Hall Of Fame shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct HOF {
    id: i32,
    name: String,
}

impl HOF {}

/// Stores a single team member shown in the "team" page
/// of this site. The is_collab field is a boolean used to indicate if the
/// team member is an outside collaborator. The status is pretty simple.
///
/// true = Outside Collaborator
/// false = UpVent Member
/// (Team)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct TeamMember {
    id: i32,
    name: String,
    position: String,
    is_collab: bool,
}

impl TeamMember {}

/// Stores a single and unique privacy policy to make this site compliant with
/// inside / outside country privacy laws (GDPR and others).
/// (Privacy Policy)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct PrivacyPolicy {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

impl PrivacyPolicy {}

/// Stores a single and unique terms of service + refund policy to make this
/// site compliant with inside / outside country trade laws.
/// (Terms Of Service)
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct TermsOfService {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

impl TermsOfService {}

// ===== Blog page =====

/// Stores a single blog post.
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    id: i32,
    published: bool,
    title: String,
    description: String,
    category: String,
    content: String,
}

impl Post {}

// ===== Marketcloud page =====

/// Stores a single marketplace product
#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    id: i32,
    name: String,
    price: f64,
    category: String,
    apptype: String,
    short_description: String,
    description: String,
    stripe_link: String,
    available: bool,
}

impl Product {}
