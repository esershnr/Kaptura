# 🚀 Kaptura 1.1.0 - Video Yakalama Yazılımı

Kaptura, Windows sistemler için geliştirilmiş, yüksek performanslı ve düşük gecikmeli bir **UVC (USB Video Class)** yakalama arayüzüdür. Oyun yakalama kartları ve profesyonel web kameralar için optimize edilmiş, minimalist ama güçlü bir araçtır.

[🇺🇸 Click here for English README](README.md)

---

## ✨ Öne Çıkan Özellikler

### 🛡️ Parazit ve Glitch Önleme
Birçok yakalama kartı, USB bant genişliği veya saat senkronizasyonu sorunları nedeniyle seste "çatırdama" veya "parazit" yapabilir. Kaptura, bunu özel bir **100ms Leaky Downstream Buffer** mimarisi kullanarak çözer. Bu sistem, gecikmeyi minimumda tutarken eskiyen paketleri atar ve sesin her zaman pürüzsüz kalmasını sağlar.

### 👤 Stealth Mode (v1.1.0 - Win32 Hayalet Modu)
Özellikle **Discord üzerinden yayın açanlar** için geliştirilen Stealth Mode, artık düşük seviyeli Win32 API çağrılarıyla en üst seviye gizlilik sunar:
- **Görünmez ve Tıklama Geçirgen**: Pencere yayın için aktif kalır ancak senin için tamamen şeffaf ve tıklanabilir (click-through) hale gelir.
- **Tam Ekran (Borderless)**: Yayın kalitesini artırmak için ekranı otomatik olarak kaplar.
- **Discord Optimizasyonu**: Teknik olarak "görünür" kalarak Discord yayınının donmasını veya duraklatılmasını engeller.

### ⚡ Akıllı Donanım Önceliği ve NV12
Kaptura, cihazları otomatik olarak tarar ve isminde **"USB3", "Capture" veya "Game"** geçen cihazlara öncelik verir ve artık ucuz yakalama kartları için optimize edilmiş **NV12 format seçimini** içerir.

### 🎨 Piksel Formatları ve Donanım Optimizasyonu
Kaptura, görüntü kalitesi ve sistem kararlılığı arasındaki dengeyi kurabilmeniz için donanım seviyesinde piksel formatı seçimi sunar:

- **NV12 (Önerilen)**: Modern yakalama kartları için en dengeli formattır. Minimum CPU yükü ile yüksek performans ve düşük gecikme sunar. Çoğu bütçe dostu kartta 1080p/60fps için en iyi seçimdir.
- **YUY2 (Zengin Görüntü)**: En zengin ve sıkıştırılmamış renk verisini sunar. Ancak, çok yüksek USB bant genişliği gerektirdiği için düşük segment kartlarda veya USB 2.0 portlarında seste "titreme" veya "patlamalara" yol açabilir. Kaptura’nın özel tamponlama mantığı bu durumu minimize etse de, sorun donanımsal bant genişliği sınırlamalarından kaynaklandığı için zaman zaman tekrarlayabilir. Donanımınız izin veriyorsa maksimum kalite için bu modu seçin.
- **MJPG (Uyumluluk)**: Donanım tabanlı sıkıştırma kullanır. Hafif görüntü kayıpları karşılığında, eski USB 2.0 kartlarda bile yüksek kare hızlarına (FPS) ulaşmak için idealdir.

### ⌨️ Kısayollar & Kontroller

| Kısayol | İşlem | Kapsam |
| :--- | :--- | :--- |
| `SHIFT + ESC` | **Stealth Mode'dan Çık** | Genel (Sadece Stealth) |
| `SHIFT + F10` | **Arayüzü Gizle/Göster** | Genel (Her Zaman) |
| `Çift Tıklama` | **Arayüzü Gizle/Göster** | Yerel (Sadece Normal) |
| `ESC` | **Uygulamayı Kapat** | Yerel (Sadece Normal) |

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
