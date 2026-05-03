# 🚀 Kaptura 1.0 - Video Yakalama Yazılımı

Kaptura, Windows sistemler için geliştirilmiş, yüksek performanslı ve düşük gecikmeli bir **UVC (USB Video Class)** yakalama arayüzüdür. Oyun yakalama kartları ve profesyonel web kameralar için optimize edilmiş, minimalist ama güçlü bir araçtır.

[🇺🇸 Click here for English README](README.md)

---

## ✨ Öne Çıkan Özellikler

### 🛡️ Parazit ve Glitch Önleme
Birçok yakalama kartı, USB bant genişliği veya saat senkronizasyonu sorunları nedeniyle seste "çatırdama" veya "parazit" yapabilir. Kaptura, bunu özel bir **100ms Leaky Downstream Buffer** mimarisi kullanarak çözer. Bu sistem, gecikmeyi minimumda tutarken eskiyen paketleri atar ve sesin her zaman pürüzsüz kalmasını sağlar.

### 👤 Stealth Mode (Discord İçin Tasarlandı)
Özellikle **Discord üzerinden yayın açanlar** için geliştirilen Stealth Mode, içeriği gizli ve temiz bir şekilde paylaşmanıza olanak tanır. Aktif edildiğinde:
- Pencere kenarlıkları ve başlık çubuğu gizlenir.
- Görev çubuğundaki görünüm minimize edilir.
- Arayüz kalabalığı olmadan, sadece saf video içeriği paylaşmak için mükemmel bir deneyim sunar.

### ⚡ Akıllı Donanım Önceliği
Kaptura, cihazları otomatik olarak tarar ve isminde **"USB3", "Capture" veya "Game"** gibi anahtar kelimeler geçen cihazlara öncelik vererek uygulamayı anında başlatır.

### 🖱️ Çift Tıklama ile UI Kontrolü
Video ekranına herhangi bir yere çift tıkladığınızda, kesintisiz bir izleme deneyimi için tüm kontrol paneli (UI) anında gizlenir veya geri gelir.

---

## 📊 Sistem Özellikleri ve Gereksinimler

- **Bellek Kullanımı:** Düşük kaynak tüketimi (Çözünürlüğe bağlı olarak ~150MB - 250MB RAM).
- **Disk Alanı:** Yaklaşık **1.5GB - 2GB** (Gerekli tüm GStreamer runtime kütüphanelerini ve DLL'leri içerir).
- **Depolama Notu:** `dist` klasörü, gerekli medya kütüphaneleri nedeniyle yüksek yer kaplamaktadır.
- **Donanım:** **Tüm UVC uyumlu yakalama kartları** ve kameralar ile uyumludur.

---

## 📦 Kurulum ve Derleme Rehberi

`dist` klasörü, boyutu nedeniyle (~2GB) repo içerisinde yer almaz. Kendi sürümünüzü oluşturmak için:

1. **GStreamer MSVC 1.24+** runtime kurun.
2. Projeyi derleyin: `cargo build --release`.
3. Bir `dist` klasörü oluşturun ve aşağıdaki yapıyı kurun:

```text
dist/
├── kaptura.exe          # target/release içerisinden derlenen dosya
├── assets/
│   └── icon.ico         # Uygulama ikonu
├── lib/
│   └── gstreamer-1.0/   # GStreamer eklentileri (runtime'dan kopyalayın)
└── *.dll                # Kritik GStreamer DLL dosyaları (runtime bin'den kopyalayın)
```

4. Yükleyiciyi oluşturmak için sağlanan `kaptura_setup.iss` dosyasını **Inno Setup** ile derleyin.

---

## 📄 Lisans
Bu proje MIT lisansı ile lisanslanmıştır.
