# Rust File Sorter
Basit bir arayüz ile dialog yardımı ile klasör seçilir ardından tekrardan bir dialog karşımıza çıkar ve kaydetmek istediğimiz lokasyonu seçtirir.

## Kullanılan Teknolojiler
Projeyi rust ile geliştirdim. Basit bir proje olmasına rağmen geliştirirken dili daha iyi anlamaya başladım. Arayüz için ise slint kullandım ve dialog açmak için ise ```rfd``` kütüphanesini kullandım.

## Projeyi Yapma Sebebim
Arkadaşım ile otururken telefonunda yer kalmamıştı ve fotoğrafları, videoları bilgisayarında yedekleyip telefonundan silecekti ancak dosyalar tarihlere göre kategorize edilmediğinden biraz sitem etti. Böylelikle bu projeyi yapmaya başladım.

## Kullanımı

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Clone the repository
    ```
    git clone https://github.com/aliyasirnac/rust_filesorter.git rust_filesorter
    cd rust_filesorter
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```

## Slint İçin Öneriler
We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).
