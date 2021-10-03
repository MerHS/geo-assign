# Geometric Modeling 21' Fall - ASSIGNMENT 2

* 2020-29856 Ho Young Jhoo

## How to run

### Windows

Run `aabb.exe`

### Linux (Ubuntu)

* Rust version

``` sh
# install libraries
sudo apt-get update
sudo apt-get install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libxcb-composite0-dev
sudo apt-get install expat libexpat-dev libfontconfig-dev libxkbcommon-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

./aabb
```

## How to play

* `Arc Split #`: 슬라이더를 조절해서 biarc의 갯수를 조절할 수 있다.
* `AABB Depth #`: 슬라이더를 조절해서 AABB 트리를 순회할 때 어느 깊이부터 AABB를 그릴지를 조절할 수 있다.
* `Use Bezier AABB`: AABB를 구할 때 arc의 AABB와 arc를 구할때 사용한 bezier curve의 sub-control point의 AABB를 merge한 것을 사용한다. (자세한 것은 후술)

## How to build

Rust를 사용하였으며 빌드가 안 된다면 다음 계정으로 메일을 보내주길 바란다. [hoyoung.jhoo@sf.snu.ac.kr](hoyoung.jhoo@sf.snu.ac.kr)

Ubuntu를 기준으로 작성하였으나 라이브러리 설치 부분만 제외하면 윈도우에서도 작동한다.

``` sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# windows는 https://www.rust-lang.org/learn/get-started 에서 rustup을 받기 바람.

# install libraries
sudo apt-get update
sudo apt-get install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev
sudo apt-get install expat libexpat-dev libfontconfig-dev libxkbcommon-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# build and run
cd rs-bezier
cargo run --release
```

## 코드 설명

* `tree.rs`: Rust로 tree의 구현과 post-order traversal을 구현함
  - Rust의 lifetime rule에 의해 실제 recursive data structure를 사용한 node의 구현은 매우 어렵다.
  - 대신 [Arena-Allocated Tree](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6) 구조를 사용하여 tree를 구현한다.
  - `bezier.rs`에서는 실제 tree 데이터를 `Rc`, `RefCell` 등의 lifetime을 runtime에 체크하는 구조체로 감싸 lifetime rule을 우회하였다.

* `bezier.rs`: `build_biarc` 함수에서 AABB를 계산하는 것을 구현하였다.

## 문제점

실제 수업시간 나온 대로 arc를 두 접선들로 이루어진 quadratic bezier curve로 근사하면 arc의 각도가 180도에 가까워질 수록 quadratic bezier curve의 control point가 발산하는 문제가 있어 AABB의 크기도 그만큼 발산하게 된다.

이 문제를 해결하기 위해 수업시간에 배운 대로 AABB의 오차를 구하지 않고, 기존에 arc를 구할 때 사용하는 bezier curve의 sub-control point, 즉, 기존 bezier curve를 C라고 하면 C(t)와 C(t-delta) 사이의 curve에 대한 control point 4개 (C(t), point0, point1, C(t+ delta))의 AABB를 구한다.

그 다음 그 sub-control point들의 AABB를 arc의 AABB와 merge 시키면 arc와 bezier curve 2개를 전부 커버하는 AABB를 구할 수 있다.

sub-control point는 어차피 AABB의 오차를 구할 때 계산해야 하는 값이므로 이 방법으로 하면 좀 더 정확하고 빠른 AABB를 구할 수 있을 것으로 생각된다.

일단 과제 설명 시간에는 AABB가 발산할 때 어떤 현상이 이루어지는지, 그리고 이를 어떻게 회피할 수 있는지 알려주었으면 좋을 것 같다.

