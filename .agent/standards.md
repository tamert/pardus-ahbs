# Kod Standartları

Pardus AHBS projesinde uygulanacak kod standartları aşağıdadır.

## Frontend (Svelte 5 + TypeScript)

### 1. Svelte 5 Snippets & Runes
- Reaktivite için modern Runes (`$state`, `$derived`, `$effect`, `$props`) kullanılmalıdır.
- Eski Svelte 4 sözdiziminden (`let`, `$:`) kaçınılmalıdır.
- Tekrar eden UI parçaları için `{#snippet}` yapısı tercih edilmelidir.

### 2. Dosya Yapısı
- Bileşen isimleri `PascalCase` olmalıdır (Örn: `PatientCard.svelte`).
- Yardımcı fonksiyonlar `utils/` klasöründe, API çağrıları `services/` klasöründe tutulmalıdır.

### 3. Styling (Tailwind CSS)
- Inline style kullanımından kaçınılmalıdır.
- Karmaşık sınıflar için `@apply` yerine Tailwind'in utility-first felsefesi korunmalıdır.
- Renk paleti `tailwind.config.js` dosyasındaki tanımlara sadık kalınarak kullanılmalıdır.

## Backend (Rust + Tauri)

### 1. Naming Conventions
- Fonksiyon ve değişken isimleri `snake_case` olmalıdır.
- Struct ve Enum isimleri `PascalCase` olmalıdır.

### 2. Error Handling
- `unwrap()` ve `expect()` mümkünse sadece testlerde veya kesinlikle hata almayacak durumlarda kullanılmalıdır.
- İş fonksiyonları `Result<T, E>` dönmeli, Tauri command'leri ise frontend'e anlamlı hata mesajları (`Result<T, String>`) iletmelidir.

### 3. Database (SQLx)
- SQL sorguları tip güvenliği için mümkünse `sqlx::query!` makrosu ile derleme aşamasında kontrol edilmelidir.
- Veritabanı işlemleri asenkron (`async`) olarak kurgulanmalıdır.

## Genel Kurallar
- **Node Versiyon Yonetimi:** Proje kökündeki `.nvmrc` dosyasına sadık kalınmalı ve `nvm use` komutu kullanılmalıdır.
- Kod her zaman `Prettier` ve `Clippy` (Rust) kurallarına göre düzenlenmiş olmalıdır.
- Türkçe değişken ismi kullanılmamalı (dokümantasyon hariç), kod dili İngilizce olmalıdır.

## Çakışma Önleme (Conflict Avoidance)
- **Modüler Yapı:** Her modül (Hasta, Aşı, Muayene) kendi `services` ve `components` klasörlerine sahip olmalıdır.
- **State Management:** Svelte 5 toplu `$state` yönetiminde çakışmaları önlemek için her modülün state'i kendi `context` veya `store` dosyasında ayrıştırılmalıdır.
- **Rust Commands:** Tauri komutları `src-tauri/src/commands` klasörü altında her modül için ayrı dosyalarda tutulmalıdır.
