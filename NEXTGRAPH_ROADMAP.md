# 🗺️ NextGraph-rs Complete Roadmap

Bu dokümantasyon NextGraph-rs örneklerini çalıştırma, broker yönetimi, kullanıcı oluşturma ve parametre ayarlama süreçlerini adım adım açıklar.

## 📋 İçindekiler

1. [Sistem Gereksinimleri](#sistem-gereksinimleri)
2. [Broker (ngd) Yönetimi](#broker-ngd-yönetimi)
3. [Admin Kullanıcı Oluşturma](#admin-kullanıcı-oluşturma)
4. [Normal Kullanıcı Oluşturma](#normal-kullanıcı-oluşturma)
5. [Rust Örnekleri Çalıştırma](#rust-örnekleri-çalıştırma)
6. [Node.js Örneği Çalıştırma](#nodejs-örneği-çalıştırma)
7. [Parametre Konfigürasyonu](#parametre-konfigürasyonu)
8. [Sorun Giderme](#sorun-giderme)
9. [Gelişmiş Kullanım](#gelişmiş-kullanım)

---

## 🔧 Sistem Gereksinimleri

### Gerekli Araçlar
```bash
# Rust kurulumu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Node.js kurulumu (Node.js örnekleri için)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install node
npm install -g pnpm

# LLVM kurulumu (derleme için gerekli)
sudo apt update && sudo apt install -y llvm-dev libclang-dev clang
```

### Repo Klonlama ve Bağımlılıklar
```bash
git clone https://github.com/waluenetwork/nextgraph-rs.git
cd nextgraph-rs
git checkout codex-guncellemeleri

# Rust bağımlılıklarını kontrol et
cargo check
```

---

## 🖥️ Broker (ngd) Yönetimi

### 1. Broker Başlatma

#### Temel Başlatma
```bash
cd nextgraph-rs
cargo run -p ngd -- -vv --save-key -l 14400
```

#### Parametreler
- `-vv`: Verbose logging (detaylı log)
- `--save-key`: Broker anahtarını kaydet
- `-l 14400`: Port 14400'de dinle
- `--help`: Tüm parametreleri göster

#### Broker Durumu Kontrol
```bash
# Broker'ın çalışıp çalışmadığını kontrol et
curl http://localhost:14400/status
# veya
netstat -tlnp | grep 14400
```

### 2. Broker Konfigürasyonu

#### Broker Peer ID'si
Broker başlatıldığında otomatik olarak bir Peer ID oluşturur:
```
Peer ID: s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA
```

Bu ID örneklerde kullanılacak.

#### Broker Verileri
```bash
# Broker verileri burada saklanır
ls -la .ng/server/
```

---

## 👑 Admin Kullanıcı Oluşturma

### 1. CLI ile Admin Davet Oluşturma

```bash
# Admin davet linki oluştur
cargo run -p ngcli -- --save-key --save-config \
  -s 127.0.0.1,14400,s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA \
  -u <ADMIN_PRIVATE_KEY> \
  admin add-invitation \
  --admin \
  --forever \
  --name "Admin User" \
  --memo "Admin invitation for examples"
```

### 2. Admin Anahtarı Alma

```bash
# Broker'ın admin anahtarını kontrol et
cat .ng/server/key
```

### 3. Web Arayüzü ile Admin Oluşturma

```bash
# Broker çalışırken bu URL'yi ziyaret et
http://localhost:14400/#/i/<INVITATION_CODE>
```

---

## 👤 Normal Kullanıcı Oluşturma

### 1. Programatik Kullanıcı Oluşturma

```rust
// create_user_cli.rs
use nextgraph::local_broker::{init_local_broker, LocalBrokerConfig};
use nextgraph::wallet::{wallet_create_v0, CreateWalletV0};
use nextgraph::net::types::BootstrapContentV0;
use ng_repo::types::PubKey;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_local_broker(Box::new(move || LocalBrokerConfig::InMemory)).await;
    
    let security_img = std::fs::read("nextgraph/examples/wallet-security-image-demo.png")?;
    let peer_id: PubKey = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA".try_into()?;
    
    let wallet_result = wallet_create_v0(CreateWalletV0 {
        security_img,
        security_txt: "Example User".to_string(),
        pin: [1, 2, 1, 2],
        pazzle_length: 9,
        send_bootstrap: false,
        send_wallet: false,
        result_with_wallet_file: true,
        local_save: true,
        core_bootstrap: BootstrapContentV0::new_localhost(peer_id),
        core_registration: None,
        additional_bootstrap: None,
        pdf: false,
        device_name: "example_device".to_string(),
    }).await?;
    
    println!("User ID: {}", wallet_result.personal_identity().to_string());
    println!("Wallet Name: {}", wallet_result.wallet_name);
    
    Ok(())
}
```

### 2. Mevcut Kullanıcıları Kullanma

```bash
# Mevcut kullanıcıları listele
ls -la .ng/example/user*/

# Kullanıcı bilgilerini kontrol et
cat .ng/example/user*/user/IDENTITY
```

---

## 🦀 Rust Örnekleri Çalıştırma

### Ön Koşullar
1. **Broker çalışıyor olmalı** (Terminal 1)
2. **Geçerli kullanıcı kimlik bilgileri** mevcut olmalı

### 1. in_memory Örneği

```bash
# Terminal 2'de çalıştır
cargo run -p nextgraph --example in_memory
```

#### Parametre Ayarları
```rust
// nextgraph/examples/in_memory.rs
let peer_id_of_server_broker: PubKey = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA".try_into().unwrap();
let user_id: PubKey = "<GENERATED_USER_ID>".try_into().unwrap();
let wallet_name = "<WALLET_NAME>".to_string();
```

### 2. persistent Örneği

```bash
cargo run -p nextgraph --example persistent
```

#### Özel Konfigürasyon
```rust
// Kalıcı depolama için
SessionConfig::new_save(&user_id, &wallet_name)
```

### 3. open Örneği

```bash
cargo run -p nextgraph --example open
```

#### Wallet Açma Parametreleri
```rust
// Pazzle ile wallet açma
let opened_wallet = wallet_open_with_pazzle(
    &wallet,
    vec![7, 17, 102, 43, 126, 84, 135, 56, 64], // Pazzle değerleri
    [1, 2, 1, 2], // PIN
)?;
```

### 4. sparql_update Örneği

```bash
cargo run -p nextgraph --example sparql_update
```

#### Kimlik Doğrulama Düzeltmesi
```rust
// Mnemonic yerine pazzle kullan
let opened_wallet = wallet_open_with_pazzle(&wallet, pazzle_values, pin)?;
```

---

## 🟨 Node.js Örneği Çalıştırma

### 1. Kurulum

```bash
cd ng-sdk-js/app-node
npm install
```

### 2. Konfigürasyon

```javascript
// index.js
const config = {
    server_peer_id: "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA",
    server_addr: "127.0.0.1:14400",
    user_id: "<GENERATED_USER_ID>",
    wallet_path: "./wallet.ngw"
};
```

### 3. Çalıştırma

```bash
npm start
```

---

## ⚙️ Parametre Konfigürasyonu

### Broker Parametreleri

| Parametre | Açıklama | Varsayılan |
|-----------|----------|------------|
| `-l, --listen` | Dinleme portu | 14400 |
| `-v, --verbose` | Log seviyesi | INFO |
| `--save-key` | Anahtarı kaydet | false |
| `--save-config` | Konfigürasyonu kaydet | false |

### Wallet Parametreleri

| Parametre | Açıklama | Tip |
|-----------|----------|-----|
| `security_txt` | Güvenlik metni | String |
| `pin` | PIN kodu | [u8; 4] |
| `pazzle_length` | Pazzle uzunluğu | u8 |
| `local_save` | Yerel kaydetme | bool |
| `device_name` | Cihaz adı | String |

### Bootstrap Konfigürasyonu

```rust
// Localhost broker için
BootstrapContentV0::new_localhost(peer_id)

// Uzak broker için
BootstrapContentV0::new_remote(peer_id, "remote.example.com:14400")
```

---

## 🔧 Sorun Giderme

### Yaygın Hatalar

#### 1. "WalletNotFound" Hatası
```bash
# Çözüm: Wallet oluştur veya mevcut wallet kullan
cargo run --bin create_user_cli
```

#### 2. "NoiseHandshakeFailed" Hatası
```bash
# Çözüm: Doğru Peer ID kullan
let peer_id: PubKey = "s2YM98jAU80Eo_l43GDnDDH33fmHc3FpE2GdCJyo5hYA".try_into().unwrap();
```

#### 3. "AccessDenied" Hatası
```bash
# Çözüm: Admin davet ile kullanıcı oluştur
cargo run -p ngcli -- admin add-invitation --admin
```

#### 4. "EncryptionError" Hatası
```bash
# Çözüm: Doğru kimlik doğrulama yöntemi kullan (pazzle vs mnemonic)
```

### Debug Komutları

```bash
# Broker loglarını kontrol et
cargo run -p ngd -- -vv --save-key -l 14400

# Kullanıcı verilerini kontrol et
ls -la .ng/example/

# Wallet dosyalarını kontrol et
file .ng/example/wallets
```

---

## 🚀 Gelişmiş Kullanım

### 1. Çoklu Kullanıcı Senaryosu

```bash
# Birden fazla kullanıcı oluştur
for i in {1..3}; do
    cargo run --bin create_user_cli -- --name "User$i"
done
```

### 2. Uzak Broker Bağlantısı

```rust
// Uzak broker için konfigürasyon
let bootstrap = BootstrapContentV0::new_remote(
    peer_id,
    "remote-broker.example.com:14400"
);
```

### 3. Veri Temizleme

```bash
# Tüm NextGraph verilerini temizle
rm -rf .ng/
```

### 4. Performans İzleme

```bash
# Broker performansını izle
cargo run -p ngd -- -vv --save-key -l 14400 2>&1 | grep -E "(connection|error|warning)"
```

---

## 📝 Örnek Çalıştırma Sırası

### Tam Workflow

```bash
# 1. Terminal 1: Broker başlat
cargo run -p ngd -- -vv --save-key -l 14400

# 2. Terminal 2: Kullanıcı oluştur
cargo run --bin create_user_cli

# 3. Çıktıdaki User ID ve Wallet Name'i kopyala

# 4. Örnekleri güncelle (User ID ve Wallet Name ile)

# 5. Örnekleri sırayla test et
cargo run -p nextgraph --example in_memory
cargo run -p nextgraph --example persistent
cargo run -p nextgraph --example open
cargo run -p nextgraph --example sparql_update

# 6. Node.js örneğini test et
cd ng-sdk-js/app-node && npm start
```

---

## 🎯 Başarı Kriterleri

### ✅ Başarılı Çalıştırma İşaretleri

1. **Broker**: Port 14400'de dinliyor
2. **Örnekler**: Hata olmadan tamamlanıyor
3. **Bağlantı**: NoiseHandshake başarılı
4. **Wallet**: Oluşturma ve açma işlemleri çalışıyor
5. **Session**: Başlatma ve durdurma başarılı

### ❌ Başarısızlık İşaretleri

1. Permission Denied hataları
2. WalletNotFound hataları
3. NoiseHandshakeFailed hataları
4. EncryptionError hataları
5. Sonsuz döngüde takılma

---

## 📞 Destek

Bu roadmap'i takip ederek NextGraph-rs örneklerini başarıyla çalıştırabilirsiniz. Her adımda karşılaştığınız sorunları yukarıdaki sorun giderme bölümünden kontrol edebilirsiniz.

**Not**: Bu dokümantasyon NextGraph-rs v0.1.1-alpha.2 sürümü için hazırlanmıştır.
