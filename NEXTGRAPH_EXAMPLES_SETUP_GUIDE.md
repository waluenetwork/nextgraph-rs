# NextGraph-rs Examples Setup Guide

Bu rehber, NextGraph-rs deposundaki örnekleri sıfırdan çalıştırmak için gereken tüm adımları detaylı olarak açıklar.

## İçindekiler

1. [Gereksinimler](#gereksinimler)
2. [Depoyu Klonlama ve Bağımlılıkları Kurma](#depoyu-klonlama-ve-bağımlılıkları-kurma)
3. [ngd Broker'ı Başlatma](#ngd-brokerı-başlatma)
4. [Admin Davet Bağlantısı Oluşturma](#admin-davet-bağlantısı-oluşturma)
5. [Örnekleri Çalıştırma](#örnekleri-çalıştırma)
6. [Kimlik Doğrulama Sorunlarını Çözme](#kimlik-doğrulama-sorunlarını-çözme)
7. [Sorun Giderme](#sorun-giderme)

## Gereksinimler

- Rust (minimum MSRV 1.74.0)
- Node.js
- LLVM
- Git

### Kurulum

```bash
# Rust kurulumu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js kurulumu (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# LLVM kurulumu (Ubuntu/Debian)
sudo apt install pkg-config gcc build-essential libglib2.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev

# Gerekli Rust araçları
cargo install cargo-watch
cargo install cargo-run-script
npm install -g pnpm
```

## Depoyu Klonlama ve Bağımlılıkları Kurma

```bash
# Depoyu klonla
git clone https://github.com/waluenetwork/nextgraph-rs.git
cd nextgraph-rs

# Frontend bağımlılıklarını kur (ngd broker için gerekli)
cd ng-sdk-js
cargo run-script app
cd ..

pnpm -C ./ng-app install
pnpm -C ./ng-app webfilebuild
pnpm -C ./helpers/app-auth install
pnpm -C ./helpers/app-auth build
```

## ngd Broker'ı Başlatma

### 1. Temiz Başlangıç (Önerilen)

Eğer daha önce ngd çalıştırdıysanız, temiz bir başlangıç için:

```bash
# Mevcut broker verilerini temizle
rm -f .ng/server/key

# ngd broker'ı başlat
cargo run -p ngd -- -vv --save-key -l 14400
```

### 2. Broker Çıktısını İnceleme

Broker başlatıldığında şu bilgileri not edin:

```
INFO  ngd] PeerId of node: 0M2-O3MjH13eaanBrw_Az8SsT90AUOWlwSBA__CLsTcA
```

Bu **Peer ID**'yi örneklerde kullanacaksınız.

### 3. Admin Davet Bağlantısını Alma

Broker ilk kez başlatıldığında otomatik olarak bir admin davet bağlantısı oluşturur:

```
Admin invitation link: http://localhost:14400/#/i/AAEAQDgAAQA3sYvw_0AgwaXlUADdT6zEz8APr8Gpad5dHyNzO77N0AEAeKPVPlEzvffLqGoXaciwsFyOFmUePR89WIo1X2P9Dc8BFXlvdXIgQnJva2VyLCBhcyBhZG1pbgA
```

Bu bağlantıyı tarayıcıda açarak admin kullanıcısı oluşturabilirsiniz.

## Admin Davet Bağlantısı Oluşturma

### CLI ile Davet Oluşturma

Eğer mevcut bir admin kullanıcınız varsa, yeni davetler oluşturabilirsiniz:

```bash
# Admin davet oluştur
cargo run -p ngcli -- --save-key --save-config \
  -s 127.0.0.1,14400,<BROKER_PEER_ID> \
  -u <ADMIN_USER_PRIVATE_KEY> \
  admin add-invitation --admin --forever --name "Example User" --memo "Test invitation" --notos
```

Parametreler:
- `<BROKER_PEER_ID>`: Broker'ın Peer ID'si (yukarıda not ettiğiniz)
- `<ADMIN_USER_PRIVATE_KEY>`: Admin kullanıcının özel anahtarı

### Kullanıcı Özel Anahtarını Bulma

Admin kullanıcı oluşturduktan sonra, özel anahtarı NextGraph uygulamasında bulabilirsiniz:
1. Wallet'ı açın
2. NextGraph logosuna tıklayın
3. "Accounts" sekmesine gidin
4. "User Private Key" değerini kopyalayın

## Örnekleri Çalıştırma

### 1. Peer ID Konfigürasyonu

Örnekleri çalıştırmadan önce, broker'ın gerçek Peer ID'sini kullanacak şekilde güncelleyin:

**nextgraph/examples/in_memory.rs:**
```rust
let peer_id_of_server_broker: PubKey = "0M2-O3MjH13eaanBrw_Az8SsT90AUOWlwSBA__CLsTcA".try_into().unwrap();
```

**nextgraph/examples/persistent.rs:**
```rust
let peer_id_of_server_broker: PubKey = "0M2-O3MjH13eaanBrw_Az8SsT90AUOWlwSBA__CLsTcA".try_into().unwrap();
```

**nextgraph/examples/open.rs:**
```rust
let peer_id_of_server_broker: PubKey = "0M2-O3MjH13eaanBrw_Az8SsT90AUOWlwSBA__CLsTcA".try_into().unwrap();
```

### 2. Rust Örneklerini Çalıştırma

Ayrı bir terminal açın ve örnekleri çalıştırın:

```bash
# Terminal 2'de (ngd broker Terminal 1'de çalışırken)
cd nextgraph-rs

# In-memory örneği
cargo run -p nextgraph --example in_memory

# Persistent örneği
cargo run -p nextgraph --example persistent

# Open örneği
cargo run -p nextgraph --example open

# SPARQL Update örneği
cargo run -p nextgraph --example sparql_update
```

### 3. Node.js Örneğini Çalıştırma

```bash
cd ng-sdk-js/app-node
npm start
```

## Kimlik Doğrulama Sorunlarını Çözme

### Beklenen Hata Mesajları

Örnekler şu hata mesajlarından birini alabilir (bu normal davranıştır):

- `AccessDenied`: Kullanıcı broker'da kayıtlı değil
- `InvitationRequired`: Davet gerekli
- `NoiseHandshakeFailed`: Peer ID uyumsuzluğu
- `ConnectionError`: Bağlantı sorunu

### Gerçek Kimlik Doğrulama için

Örneklerin gerçek işlevselliği görmesi için:

1. **Admin davet bağlantısını kullanarak wallet oluşturun**
2. **Kullanıcı kimlik bilgilerini çıkarın**
3. **Örnekleri gerçek kimlik bilgileriyle güncelleyin**

Örnek güncelleme:
```rust
// PubKey::nil() yerine gerçek kullanıcı kimlik bilgilerini kullanın
let user_priv_key = PrivKey::from_base58("<EXTRACTED_USER_PRIVATE_KEY>").unwrap();
let user_id = user_priv_key.to_pub();
```

## Sorun Giderme

### Permission Denied Hataları

**Sorun:** `Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }`

**Çözüm:** ngd broker'ın çalıştığından emin olun. Örnekler broker olmadan çalışmaz.

### NoiseHandshakeFailed Hataları

**Sorun:** Peer ID uyumsuzluğu

**Çözüm:** 
1. Broker'ın gerçek Peer ID'sini kontrol edin
2. Örneklerdeki `PubKey::nil()` kullanımlarını gerçek Peer ID ile değiştirin

### EncryptionError (sparql_update örneği)

**Sorun:** Wallet kimlik doğrulama uyumsuzluğu

**Çözüm:** 
1. Wallet'ın pazzle ile mi mnemonic ile mi oluşturulduğunu kontrol edin
2. Örneği doğru kimlik doğrulama yöntemiyle güncelleyin

### Broker Bağlantı Sorunları

**Sorun:** Broker'a bağlanılamıyor

**Çözüm:**
1. Broker'ın localhost:14400'de çalıştığını kontrol edin
2. Firewall ayarlarını kontrol edin
3. Broker loglarını inceleyin

## Gelişmiş Kullanım

### Birden Fazla Kullanıcı Oluşturma

```bash
# Yeni davet oluştur
cargo run -p ngcli -- -s 127.0.0.1,14400,<PEER_ID> -u <ADMIN_KEY> admin add-invitation --notos

# Kullanıcıları listele
cargo run -p ngcli -- -s 127.0.0.1,14400,<PEER_ID> -u <ADMIN_KEY> admin list-users -a
```

### Test Verilerini Temizleme

```bash
# Tüm broker verilerini temizle
rm -rf .ng/

# Sadece server verilerini temizle
rm -rf .ng/server/
```

## Notlar

- Bu örnekler eğitim amaçlıdır ve gerçek üretim kullanımı için tasarlanmamıştır
- Broker'ın çalışır durumda olması tüm örnekler için gereklidir
- Kimlik doğrulama hataları NextGraph'ın güvenlik mimarisinin doğru çalıştığını gösterir
- Gerçek işlevsellik için uygun kullanıcı kaydı ve kimlik doğrulama gereklidir

## Ek Kaynaklar

- [NextGraph Dokümantasyonu](https://docs.nextgraph.org/)
- [DEV.md](DEV.md) - Geliştirici rehberi
- [ngd README](ngd/README.md) - Broker dokümantasyonu
- [ngcli README](ngcli/README.md) - CLI araç dokümantasyonu
