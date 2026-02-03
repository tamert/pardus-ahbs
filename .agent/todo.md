# AHBS Projesi Geliştirme Planı

## 🏗️ Temel Yapı
- [x] Proje temel yapısının kurulması (Tauri 2 + Svelte 5 + Tailwind CSS)
- [x] Veritabanı altyapısının kurgulanması (SQLite + SQLx)
- [x] Modern ve premium UI taslak tasarımı
- [ ] SQLite Migration yapısının düzenlenmesi
- [ ] Kimlik doğrulama (Doktor girişi)

## 💉 Aşı İşlemleri Modülü (Wiki Detayları)
- [ ] **Aşı Takvimi Mantığı**
    - [ ] Bebek, çocuk ve gebe aşı takvimlerinin yaşa/periyoda göre listelenmesi.
    - [ ] Doğum tarihine göre otomatik aşı zamanı hesaplama algoritmaları.
- [ ] **Aşı Kayıt ve Uygulama**
    - [ ] Aşı kayıt formu (Doz no, uygulama yeri, uygulama şekli vb. alanlar).
    - [ ] ATS (Aşı Takip Sistemi) entegrasyon simülasyonu (Karekod sorgulama).
    - [ ] Aşı stok düşüm mantığının kurulması.
- [ ] **Erteleme ve İptal Süreçleri**
    - [ ] Tıbbi nedenlerle veya hasta tercihiyle erteleme kayıtları.
- [ ] **İstenmeyen Etki (ASİE)**
    - [ ] Aşı sonrası gelişen yan etkilerin (Ateş, şişlik vb.) kayıt ekranı.
- [ ] **Geçmiş Sorgulama**
    - [ ] USS/SağlıkNet üzerinden hastanın geçmiş aşılarını çekme (Mock API).

## 🩺 Muayene ve Hasta Takip
- [ ] Hasta kayıt modülü için Rust Command ve UI bağlantısı
- [ ] Muayene ekranı geliştirmesi (Tanı, Reçete, İşlem)
- [ ] Reçete sistemi

## 📊 Raporlama ve Analiz
- [ ] Günlük/Aylık çalışma raporları
- [ ] İstatistiksel verilerin görselleştirilmesi
