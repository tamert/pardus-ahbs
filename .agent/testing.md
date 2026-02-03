# Test Standartları ve Stratejisi

AHBS gibi kritik bir sistemde testler hayati önem taşır.

## 1. Unit Testler (Backend - Rust)
- Her kritik iş mantığı (Örn: Aşı takvimi hesaplama) için `#[test]` attribute'u ile birim testleri yazılmalıdır.
- Komut: `cargo test`

## 2. Integration Testler (Database)
- SQLite sorgularının doğruluğunu teyit etmek için `sqlx` üzerinden DB entegrasyon testleri kurgulanmalıdır.
- Test için ayrı bir `test.db` kullanılmalıdır.

## 3. Frontend Component Testleri (Vitest)
- Svelte bileşenlerinin doğru render edilmesi ve kullanıcı etkileşimlerini (click, input) test etmek için `Vitest` kullanılmalıdır.
- Komut: `npm run test:unit`

## 4. End-to-End (E2E) Testleri (Playwright)
- Kritik akışlar (Örn: Hasta kaydı -> Muayene açılması -> Reçete yazılması) Playwright ile test edilmelidir.
- Komut: `npm run test:e2e`

## Test Kapsamı Hedefi
- İş mantığı (Business Logic) kapsama oranı: **%90+**
- UI bileşen kapsama oranı: **%70+**

## Test Yazım Kuralı
- Test isimleri neyi test ettiğini açıkça belirtmelidir (Örn: `test_child_vaccination_schedule_calculation`).
- Her PR (Pull Request) öncesi testlerin lokalde hatasız çalıştığı kontrol edilmelidir.
