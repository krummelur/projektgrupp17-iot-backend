use std::env;
use colour;
static ENVIRONMENT_VAR:   &'static str = "RUST_IOT_ENVIRONMENT";
static PRODUCTION_STRING: &'static str =  "PRODUCTION" ;
static TEST_STRING:       &'static str =  "TEST";

struct DbVars {
    host_var: &'static str,
    db_var: &'static str , 
    user_var: &'static str ,
    pass_var: &'static str
}

impl DbVars {
    fn new_test() -> DbVars {
        DbVars{ 
            host_var: "SQL_HOST_TEST",
            db_var: "SQL_DB_NAME_TEST",
            user_var: "SQL_USERNAME_TEST",
            pass_var: "SQL_PASSWORD_TEST"
        }   
    }
    fn new_prod() -> DbVars {
        DbVars {
            host_var: "SQL_HOST",
            db_var: "SQL_DB_NAME",
            user_var: "SQL_USERNAME",
            pass_var: "SQL_PASSWORD"   
        }
    }
}

pub struct DbValues {
    pub host: String,
    pub db_name: String,
    pub user: String,
    pub pass: String
}

pub fn db_environment_values() -> DbValues {
    let cur_env_val = env::var(ENVIRONMENT_VAR).unwrap_or_else(|_| {println!("environment setting not found, using test environment"); return TEST_STRING.to_owned()}); 
    
    let is_production = String::from(PRODUCTION_STRING) == cur_env_val;
    println!("{}, {}, is equal: {}", String::from(PRODUCTION_STRING), cur_env_val, is_production);
    
    match is_production {
        false => colour::yellow!("\n### USING STAGING ENVIRONMENT (not an error) \n\n"),
        true =>  colour::dark_red!("\n### WARNING! USING PRODUCTION ENVIRONMENT ###\n\n")
    }

    let current_vars = match is_production {
        false => DbVars::new_test(),
        true =>  DbVars::new_prod()
    };

    let host = env::var(current_vars.host_var ).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", current_vars.host_var)));
    let db_name = env::var(current_vars.db_var).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", current_vars.db_var  )));
    let user = env::var(current_vars.user_var ).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", current_vars.user_var)));
    let pass = env::var(current_vars.pass_var ).unwrap_or_else(|_| panic!(format!("Error reading environment variable {}", current_vars.pass_var)));

    DbValues {
        host: host,
        db_name: db_name,
        user: user,
        pass: pass
    }
}