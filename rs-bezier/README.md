# Geometric Modeling 21' Fall - ASSIGNMENT 1

* 2020-29856 Ho Young Jhoo

## How to run

### Windows

Run `bezier.exe` or `bezier_cpp.exe`


### Linux (Ubuntu)

* Rust version

``` sh
# install libraries
sudo apt-get update
sudo apt-get install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev
sudo apt-get install expat libexpat-dev libfontconfig-dev libxkbcommon-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

./bezier.out
```

* C++ version

Run `bezier_cpp.out`

Ubuntu 20.04 LTS x64 에서 빌드하였다. 다른 리눅스 배포판에서 빌드하려면 아래를 참고.

## How to build 

주 언어로 Rust를 사용하였다. Rust 빌드가 안되는 경우를 고려해서 C++ 버전도 제작하였으며 빌드가 잘 된다면 다음 숙제부터 Rust 코드만을 제출할 계획이므로 빌드가 안된다면 다음 계정으로 피드백을 보내주기 바란다. (hoyoung.jhoo@sf.snu.ac.kr) 

Ubuntu를 기준으로 작성.

``` sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install libraries
sudo apt-get update
sudo apt-get install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev
sudo apt-get install expat libexpat-dev libfontconfig-dev libxkbcommon-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# build and run
cd rs-bezier
cargo run --release

```

### How to build C++ project

``` sh
cd bezier-cpp
make
```

## 코드 설명

고급 라이브러리를 사용해서 자동으로 그려냈다는 의혹을 방지하기 위해 간단히 Rust 코드를 설명한다. 

사용한 라이브러리는 Rust OpenGL wrapper중 하나인 glium과 간편한 cross-platform 폰트 작업 및 UI, 윈도우를 관리하기 위한 conrod, winit을 사용하였다. 




