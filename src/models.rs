use rocket::serde::Deserialize;
use rocket::serde::Serialize;

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
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct License {
    id: i32,
    name: String,
    verbatim: String,
    license_link: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "licenses"]
pub struct NewLicense {
    name: String,
    verbatim: String,
    license_link: String,
}

impl License {
    /// Returns a vector consisting of a single License in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first license saved in the database
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
    /// let first = License::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<License> {
        all_licenses
            .find(id)
            .load::<License>(conn)
            .expect("Error loading license")
    }

    /// Returns a vector of all Licenses saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all licenses saved in the database
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
    /// let test_list = License::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<License> {
        all_licenses
            .order(licenses::id.desc())
            .load::<License>(conn)
            .expect("Error loading licenses")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, lic: NewLicense) -> bool {
        use crate::schema::licenses::dsl::{license_link as l, name as n, verbatim as v};

        let NewLicense {
            name,
            verbatim,
            license_link,
        } = lic;

        diesel::update(all_licenses.find(id))
            .set((n.eq(name), v.eq(verbatim), l.eq(license_link)))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * lic: A `NewLicense` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the Licenses table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let license = models::NewLicense {
    ///    name: String::from("GPL-2"),
    ///    verbatim: String::from("No proprietary sotfwaer!"),
    ///    license_link: String::from("https://gnu.org/"),
    /// };
    ///
    /// if models::License::insert(license, &conn) {
    ///    println!("License inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the License!");
    /// }
    /// ```
    ///
    pub fn insert(lic: NewLicense, conn: &SqliteConnection) -> bool {
        diesel::insert_into(licenses::table)
            .values(&lic)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if License::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_testimonials.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<License> {
        all_licenses
            .filter(licenses::name.eq(name))
            .load::<License>(conn)
            .expect("Error loading licenses by name")
    }
}

/// Stores a single Hall Of Fame shown in the "licenses" page
/// of this site.
/// (Licenses)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct HOF {
    id: i32,
    name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "hofs"]
pub struct NewHOF {
    name: String,
}

impl HOF {
    /// Returns a vector consisting of a single Hall Of Fame Project in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first hall of fame project saved in the database
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
    /// let first = HOF::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<HOF> {
        all_hofs
            .find(id)
            .load::<HOF>(conn)
            .expect("Error loading hall of fame project")
    }

    /// Returns a vector of all hall of fame projects saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all hall of fame projects saved in the database
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
    /// let test_list = HOF::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<HOF> {
        all_hofs
            .order(hofs::id.desc())
            .load::<HOF>(conn)
            .expect("Error loading hall of fame projects")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, hof: NewHOF) -> bool {
        use crate::schema::hofs::dsl::name as n;

        let NewHOF { name } = hof;

        diesel::update(all_hofs.find(id))
            .set(n.eq(name))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * lic: A `NewHOF` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the HOF table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let hof = models::NewHOF {
    ///    name: String::from("Debian"),
    /// };
    ///
    /// if models::HOF::insert(hof, &conn) {
    ///    println!("Hall Of Fame Project inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the Hall Of Fame Project!");
    /// }
    /// ```
    ///
    pub fn insert(hof: NewHOF, conn: &SqliteConnection) -> bool {
        diesel::insert_into(hofs::table)
            .values(&hof)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if HOF::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_hofs.find(id)).execute(conn).is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<HOF> {
        all_hofs
            .filter(hofs::name.eq(name))
            .load::<HOF>(conn)
            .expect("Error loading Hall Of Fame Projects by name")
    }
}

/// Stores a single team member shown in the "team" page
/// of this site. The is_collab field is a boolean used to indicate if the
/// team member is an outside collaborator. The status is pretty simple.
///
/// true = Outside Collaborator
/// false = UpVent Member
/// (Team)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TeamMember {
    id: i32,
    name: String,
    position: String,
    is_collab: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "teammembers"]
pub struct NewTeamMember {
    name: String,
    position: String,
    is_collab: bool,
}

impl TeamMember {
    /// Returns a vector consisting of a single Team Member in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first team member saved in the database
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
    /// let first = TeamMember::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<TeamMember> {
        all_teammembers
            .find(id)
            .load::<TeamMember>(conn)
            .expect("Error loading team member")
    }

    /// Returns a vector of all team members saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all team members saved in the database
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
    /// let test_list = TeamMember::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<TeamMember> {
        all_teammembers
            .order(teammembers::id.desc())
            .load::<TeamMember>(conn)
            .expect("Error loading team members")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, teamm: NewTeamMember) -> bool {
        use crate::schema::teammembers::dsl::{is_collab as c, name as n, position as p};

        let NewTeamMember {
            name,
            position,
            is_collab,
        } = teamm;

        diesel::update(all_teammembers.find(id))
            .set((n.eq(name), p.eq(position), c.eq(is_collab)))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * lic: A `NewHOF` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the HOF table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let teamm = models::NewTeamMember {
    ///    name: String::from("Juan Camaney"),
    ///    position: String::from("Gum chewer & hard hitter"),
    ///    is_collab: false
    /// };
    ///
    /// if models::TeamMember::insert(teamm, &conn) {
    ///    println!("Team Member inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the team member!");
    /// }
    /// ```
    ///
    pub fn insert(teamm: NewTeamMember, conn: &SqliteConnection) -> bool {
        diesel::insert_into(teammembers::table)
            .values(&teamm)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if TeamMember::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_teammembers.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<TeamMember> {
        all_teammembers
            .filter(teammembers::name.eq(name))
            .load::<TeamMember>(conn)
            .expect("Error loading team members by name")
    }
}

/// Stores a single and unique privacy policy to make this site compliant with
/// inside / outside country privacy laws (GDPR and others).
/// (Privacy Policy)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct PrivacyPolicy {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "privacypolicies"]
pub struct NewPrivacyPolicy {
    title: String,
    changelog: String,
    text: String,
}

impl PrivacyPolicy {
    /// Returns a vector consisting of a single Privacy Policy in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The privacy policy ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first hall of fame project saved in the database
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
    /// let first = PrivacyPolicy::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<PrivacyPolicy> {
        all_privacypolicies
            .find(id)
            .load::<PrivacyPolicy>(conn)
            .expect("Error loading privacy policy")
    }

    /// Returns a vector of all privacy policies saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all privacy policies saved in the database
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
    /// let test_list = PrivacyPolicy::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<PrivacyPolicy> {
        all_privacypolicies
            .order(privacypolicies::id.desc())
            .load::<PrivacyPolicy>(conn)
            .expect("Error loading privacy policies")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, ppol: NewPrivacyPolicy) -> bool {
        use crate::schema::privacypolicies::dsl::{changelog as c, text as tx, title as t};

        let NewPrivacyPolicy {
            title,
            changelog,
            text,
        } = ppol;

        diesel::update(all_privacypolicies.find(id))
            .set((t.eq(title), c.eq(changelog), tx.eq(text)))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * ppol: A `NewPrivacyPolicy` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the PrivacyPolicy table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let ppol = models::NewPrivacyPolicy {
    ///    title: String::from("Privacy Policy - 1984"),
    ///    changelog: String::from("This privacy policy was modified to meet the standandards of 1984"),
    ///    text: String::from("Bottom Text"),
    /// };
    ///
    /// if models::PrivacyPolicy::insert(ppol, &conn) {
    ///    println!("Privacy policy inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the Privacy Policy!");
    /// }
    /// ```
    ///
    pub fn insert(ppol: NewPrivacyPolicy, conn: &SqliteConnection) -> bool {
        diesel::insert_into(privacypolicies::table)
            .values(&ppol)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if PrivacyPolicy::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_privacypolicies.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_title(title: String, conn: &SqliteConnection) -> Vec<PrivacyPolicy> {
        all_privacypolicies
            .filter(privacypolicies::title.eq(title))
            .load::<PrivacyPolicy>(conn)
            .expect("Error loading Privacy Policies by title")
    }
}

/// Stores a single and unique terms of service + refund policy to make this
/// site compliant with inside / outside country trade laws.
/// (Terms Of Service)
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct TermsOfService {
    id: i32,
    title: String,
    changelog: String,
    text: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "termsofservices"]
pub struct NewTermsOfService {
    title: String,
    changelog: String,
    text: String,
}

impl TermsOfService {
    /// Returns a vector consisting of a single Terms Of Service instance in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first terms of service instance saved in the database
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
    /// let first = TermsOfService::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<TermsOfService> {
        all_termsofservices
            .find(id)
            .load::<TermsOfService>(conn)
            .expect("Error loading Terms Of Service")
    }

    /// Returns a vector of all terms of service saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all hall of fame projects saved in the database
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
    /// let test_list = TermsOfService::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<TermsOfService> {
        all_termsofservices
            .order(termsofservices::id.desc())
            .load::<TermsOfService>(conn)
            .expect("Error loading terms of service")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, tos: NewTermsOfService) -> bool {
        use crate::schema::termsofservices::dsl::{changelog as c, text as tx, title as t};

        let NewTermsOfService {
            title,
            changelog,
            text,
        } = tos;

        diesel::update(all_termsofservices.find(id))
            .set((t.eq(title), c.eq(changelog), tx.eq(text)))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * tos: A `NewTermsOfService` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the HOF table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let tos = models::NewTermsOfService {
    ///    title: String::from("Terms of service"),
    ///    changelog: String::from("Changes to comply with 1984 policies"),
    ///    text: String::from("I forgor"),
    /// };
    ///
    /// if models::TermsOfService::insert(tos, &conn) {
    ///    println!("Terms Of Service inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting terms of service!");
    /// }
    /// ```
    ///
    pub fn insert(tos: NewTermsOfService, conn: &SqliteConnection) -> bool {
        diesel::insert_into(termsofservices::table)
            .values(&tos)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if TermsOfService::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_termsofservices.find(id))
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn all_by_title(title: String, conn: &SqliteConnection) -> Vec<TermsOfService> {
        all_termsofservices
            .filter(termsofservices::title.eq(title))
            .load::<TermsOfService>(conn)
            .expect("Error loading Terms Of Service by name")
    }
}

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

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct NewPost {
    published: bool,
    title: String,
    description: String,
    category: String,
    content: String,
}

impl Post {
    /// Returns a vector consisting of a single blog post in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first hall of fame project saved in the database
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
    /// let first = Post::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Post> {
        all_posts
            .find(id)
            .load::<Post>(conn)
            .expect("Error loading blog post")
    }

    /// Returns a vector of all blog posts saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all blog posts saved in the database
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
    /// let test_list = Post::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<Post> {
        all_posts
            .order(posts::id.desc())
            .load::<Post>(conn)
            .expect("Error loading blog posts")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, pos: NewPost) -> bool {
        use crate::schema::posts::dsl::{
            category as c, content as cn, description as d, published as p, title as t,
        };

        let NewPost {
            published,
            title,
            description,
            category,
            content,
        } = pos;

        diesel::update(all_posts.find(id))
            .set((
                p.eq(published),
                t.eq(title),
                d.eq(description),
                c.eq(category),
                cn.eq(content),
            ))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * pos: A `NewPost` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the HOF table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let pos = models::NewPost {
    ///     published: true,
    ///     title: String::from("Como crear un CRUD con Rust + Diesel"),
    ///     description: String::from("I forgor"),
    ///     category: String::from("Tutoriales"),
    ///     content: String::from("...."),
    /// };
    ///
    /// if models::Post::insert(pos, &conn) {
    ///    println!("Blog Post inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the Blog post!");
    /// }
    /// ```
    ///
    pub fn insert(pos: NewPost, conn: &SqliteConnection) -> bool {
        diesel::insert_into(posts::table)
            .values(&pos)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if Post::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_hofs.find(id)).execute(conn).is_ok()
    }

    /// Documentation pending
    pub fn all_by_title(title: String, conn: &SqliteConnection) -> Vec<Post> {
        all_posts
            .filter(posts::title.eq(title))
            .load::<Post>(conn)
            .expect("Error loading products by title")
    }
}

// ===== Marketcloud page =====

/// Stores a single marketplace product
#[derive(Serialize, Queryable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    id: i32,
    name: String,
    price: f32,
    category: String,
    apptype: String,
    short_description: String,
    description: String,
    stripe_link: String,
    available: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "products"]
pub struct NewProduct {
    name: String,
    price: f32,
    category: String,
    apptype: String,
    short_description: String,
    description: String,
    stripe_link: String,
    available: bool,
}

impl Product {
    /// Returns a vector consisting of a single Product in the current
    /// database.
    ///
    /// # Arguments
    ///
    /// * `id`: The license ID you wish to show
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get the first product saved in the database
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
    /// let first = Product::show(1, sqlite_connection);
    ///
    /// println!("{:?}", first);
    /// ```
    ///
    pub fn show(id: i32, conn: &SqliteConnection) -> Vec<Product> {
        all_products
            .find(id)
            .load::<Product>(conn)
            .expect("Error loading product")
    }

    /// Returns a vector of all products saved in the current database
    ///
    /// # Arguments
    ///
    /// * `conn`: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Get all hall of fame projects saved in the database
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
    /// let test_list = Product::all(sqlite_connection);
    ///
    /// println!("{:?}", test_list);
    /// ```
    ///
    pub fn all(conn: &SqliteConnection) -> Vec<Product> {
        all_products
            .order(products::id.desc())
            .load::<Product>(conn)
            .expect("Error loading products")
    }

    /// Documentation pending
    pub fn update_by_id(id: i32, conn: &SqliteConnection, prod: NewProduct) -> bool {
        use crate::schema::products::dsl::{
            apptype as a, available as av, category as c, description as d, name as n, price as p,
            short_description as s, stripe_link as st,
        };

        let NewProduct {
            name,
            price,
            category,
            apptype,
            short_description,
            description,
            stripe_link,
            available,
        } = prod;

        diesel::update(all_products.find(id))
            .set((
                n.eq(name),
                p.eq(price),
                c.eq(category),
                a.eq(apptype),
                s.eq(short_description),
                d.eq(description),
                st.eq(stripe_link),
                av.eq(available),
            ))
            .execute(conn)
            .is_ok()
    }

    /// Returns a boolean if the resulting insert (add) operation was
    /// excecuted successfully.
    ///
    /// # Arguments
    ///
    /// * prod: A `NewProduct` struct to insert
    /// * conn: A reference to an SQLite Connection
    ///
    /// # Examples
    ///
    /// ```
    /// // Insert a record into the Product table
    ///
    /// let db_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    /// let conn = SqliteConnection::establish(&db_url).unwrap();
    ///
    /// let prod = models::NewProduct {
    ///    name: String::from("UpVent RS (Rust Svelte) Server"),
    ///    price: 0.0,
    ///    category: String::from("Software"),
    ///    apptype: String::from("Web"),
    ///    short_description: String::from("A small website server written in Rust and Svelte"),
    ///    description: String::from("I forgor 💀"),
    ///    stripe_link: String::from("https://stripe.com/"),
    ///    available: false, // still in early alpha xD
    /// };
    ///
    /// if models::Product::insert(prod, &conn) {
    ///    println!("Product inserted correctly!");
    /// } else {
    ///    println!("Something failed while inserting the Product!");
    /// }
    /// ```
    ///
    pub fn insert(prod: NewProduct, conn: &SqliteConnection) -> bool {
        diesel::insert_into(products::table)
            .values(&prod)
            .execute(conn)
            .is_ok()
    }

    /// Documentation pending
    pub fn delete_by_id(id: i32, conn: &SqliteConnection) -> bool {
        if HOF::show(id, conn).is_empty() {
            return false;
        };

        diesel::delete(all_products.find(id)).execute(conn).is_ok()
    }

    /// Documentation pending
    pub fn all_by_name(name: String, conn: &SqliteConnection) -> Vec<Product> {
        all_products
            .filter(products::name.eq(name))
            .load::<Product>(conn)
            .expect("Error loading Products by name")
    }
}
