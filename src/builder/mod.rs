pub mod cargo;
pub mod cmake;
pub mod make;


pub enum BuildSystem {
    Cargo,
    CMake,
    Make,
}


pub fn build_project(system: BuildSystem) {

    match system {

        BuildSystem::Cargo =>
            cargo::build(),

        BuildSystem::CMake =>
            cmake::build(),

        BuildSystem::Make =>
            make::build(),
    }
}