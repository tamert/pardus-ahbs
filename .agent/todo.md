# AHBS Projesi Geliştirme Planı

## 🏗️ Temel Yapı
- [x] Proje temel yapısının kurulması (Tauri 2 + Svelte 5 + Carbon Design System)
- [x] Veritabanı altyapısının kurgulanması (SQLite + SQLx)
- [x] Dark/Light mod desteği (Carbon modunda)
- [ ] SQLite Migration yapısının düzenlenmesi
- [ ] Kimlik doğrulama (Doktor girişi)

## 💉 Aşı İşlemleri Modülü (Wiki Detayları)
- [x] **Aşı Takvimi Mantığı**
    - [x] Bebek, çocuk ve gebe aşı takvimlerinin yaşa/periyoda göre listelenmesi.
    - [x] Doğum tarihine göre otomatik aşı zamanı hesaplama algoritmaları.
- [ ] **Aşı Kayıt ve Uygulama**
    - [ ] Aşı Uygulama Ekranı (Doz no, uygulama yeri, LOT/Seri No).
    - [ ] ATS (Aşı Takip Sistemi) entegrasyon simülasyonu (Karekod/Barkod okuma).
    - [ ] Aşı stok düşüm mantığı.
- [ ] **Erteleme ve İptal Süreçleri**
    - [ ] Tıbbi nedenlerle veya hasta tercihiyle erteleme kayıtları.
- [ ] **İstenmeyen Etki (ASİE)**
    - [ ] Aşı sonrası gelişen yan etkilerin (Ateş, şişlik vb.) kayıt ekranı.
- [ ] **Geçmiş Sorgulama**
    - [ ] USS/SağlıkNet üzerinden hastanın geçmiş aşılarını çekme (Mock API).

## 🩺 Muayene ve Hasta Takip
- [x] Hasta kayıt modülü için Rust Command ve UI bağlantısı
- [x] Hasta listeleme ve arama fonksiyonları
- [x] Detaylı Muayene ekranı geliştirmesi (Tanı, Fiziksel Bulgular, ICD-10)
- [x] Dinamik Reçete sistemi ve geçmiş muayene takibi
- [ ] Klinik karar destek sistemleri entegrasyonu (İlaç etkileşimi vb.)

## 🎨 UI/UX ve Modernizasyon
- [x] IBM Carbon Design System entegrasyonu
- [x] Özgür AHBS markalaması ve Antigravity Premium UI standartları
- [x] Dinamik modül bazlı navigasyon ve ribbon tasarımı
- [ ] Gelişmiş veri görselleştirme (Dashboard grafikleri)
- [ ] Mobil uyumluluk ve responsive optimizasyonlar
