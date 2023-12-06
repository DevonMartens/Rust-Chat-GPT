use ai_functions::ai_function;

#[ai_function]
pub fn print_project_scope(_project_description: &str) {
    println!("Project scope is printed");
}

#[ai_function]
pub fn print_site_urls(_project_description: &str) {
    println!("Project description is printed");
}