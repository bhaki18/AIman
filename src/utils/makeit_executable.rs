use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn makeit_executable(path:String) {
     #[cfg(target_os = "linux")]
    {
        let mut perms = fs::metadata(&path).unwrap().permissions();

        // chmod +x
        perms.set_mode(perms.mode() | 0o111);

        fs::set_permissions(path, perms).unwrap();
    }

    #[cfg(target_os = "windows")]
    {   
        if !path.ends_with(".exe"){
        fs::rename(path,path + ".exe");
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mut perms = fs::metadata(path).unwrap().permissions();

        // chmod +x
        perms.set_mode(perms.mode() | 0o111);

        fs::set_permissions(path, perms).unwrap();
    }
    


}