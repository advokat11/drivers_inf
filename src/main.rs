use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Функция для установки драйвера из .inf файла
fn install_driver(inf_path: &Path) {
    // Запускаем команду pnputil.exe с параметрами -i -a и путем к .inf файлу
    let output = Command::new("pnputil.exe")
        .args(&["-i", "-a", inf_path.to_str().unwrap()])
        .output()
        .expect("Ошибка запуска pnputil.exe");

    // Проверяем код возврата и вывод команды
    if output.status.success() {
        println!("Успешно установлен драйвер из {}", inf_path.display());
    } else {
        println!("Не удалось установить драйвер из {}", inf_path.display());
        println!("Код ошибки: {}", output.status.code().unwrap());
        println!("Вывод команды: {}", String::from_utf8_lossy(&output.stderr));
    }
}

// Функция для рекурсивного поиска .inf файлов в папке и подпапках
fn find_inf_files(dir: &Path) {
    // Получаем список файлов и папок в текущей папке
    let entries = fs::read_dir(dir).expect("Ошибка чтения папки");

    // Перебираем все элементы в списке
    for entry in entries {
        // Получаем путь к элементу
        let path = entry.expect("Ошибка получения пути").path();

        // Если это файл с расширением .inf, то вызываем функцию для установки драйвера
        if path.is_file() && path.extension().unwrap_or_default() == "inf" {
            install_driver(&path);
        }

        // Если это папка, то рекурсивно вызываем функцию для поиска .inf файлов в ней
        if path.is_dir() {
            find_inf_files(&path);
        }
    }
}

// Главная функция программы
fn main() {
    // Получаем начальную папку из аргументов командной строки или из текущей рабочей директории
    let start_dir = env::args().nth(1).unwrap_or_else(|| env::current_dir().expect("Ошибка получения текущей директории").to_str().unwrap().to_string());

    // Преобразуем начальную папку в объект Path
    let start_dir = Path::new(&start_dir);

    // Вызываем функцию для рекурсивного поиска .inf файлов в начальной папке и подпапках
    find_inf_files(start_dir);
}