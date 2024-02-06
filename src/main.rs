extern crate chrono;
use chrono::{ DateTime, Utc };
use std::{ fs, path::PathBuf, time };
use rfd::FileDialog;

slint::include_modules!();

#[derive(Debug, Clone)]
struct Properties {
    path: Option<PathBuf>,
    created_at: String,
}

fn to_readable_date(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%Y %B"))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.on_select_files({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            println!("{:}", ui.get_isFolder());
            let mut paths = Vec::<Properties>::new();
            let folders = FileDialog::new().pick_folders().unwrap();

            for folder in folders {
                let folder_path = folder.as_path();
                println!("folder path: {}", folder_path.display());

                let Some(path_str) = folder.as_path().to_str() else {
                    continue;
                };
                let metadata = fs::metadata(path_str.to_string());
                match metadata.ok().unwrap().created() {
                    Ok(_) => {
                        let files = fs::read_dir(folder_path).unwrap();

                        for file in files {
                            let file_entry = file.unwrap();
                            let file_path = file_entry.path();
                            let file_metadata = fs::metadata(&file_path).expect("file not found!");

                            paths.push(Properties {
                                path: Some(file_path.into()),
                                created_at: to_readable_date(
                                    &file_metadata.created().ok().unwrap()
                                ),
                            });
                        }

                        if let Some(selected_folder) = FileDialog::new().pick_folder() {
                            for path in &paths {
                                let date = to_readable_date(&time::SystemTime::now());
                                println!(
                                    "path date: {:} variable date: {:}",
                                    path.created_at,
                                    date
                                );

                                if path.created_at != date {
                                    println!("yook");
                                    continue; // Bu dosyayı atla, fakat döngüyü sonlandırma
                                }

                                // Burada, hedef klasör yolu oluşturulurken doğru metod kullanılmalı
                                let target_path = selected_folder.join(path.created_at.to_string());

                                println!("{}", target_path.display());
                                if !target_path.exists() {
                                    fs::create_dir(&target_path).expect(
                                        "Klasör oluşturulurken hata oluştu"
                                    );
                                }

                                if let Some(source_path) = &path.path {
                                    let target_file_path = target_path.join(
                                        source_path.file_name().unwrap()
                                    );
                                    match fs::copy(source_path, &target_file_path) {
                                        Ok(_) =>
                                            println!(
                                                "Dosya başarıyla kopyalandı: {}",
                                                target_file_path.display()
                                            ),
                                        Err(e) =>
                                            println!("Dosya kopyalanırken hata oluştu: {:?}", e),
                                    }
                                }
                            }
                        } else {
                            println!("Kullanıcı klasör seçmedi.");
                        }
                    }
                    Err(error) =>
                        println!(
                            "Warning: A folder path could not be converted to a UTF-8 string and was skipped.\nError: {error:?}"
                        ),
                }
            }
        }
    });

    ui.run()
}
