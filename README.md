# Geometric Modeling 21' Fall - ASSIGNMENT 1

## How to run

### Windows

Run `bezier.exe` or `bezier-button.exe`


### Linux (Ubuntu)

* Rust version

``` sh
# install libraries
sudo apt-get update
sudo apt-get install pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libxcb-composite0-dev
sudo apt-get install expat libexpat-dev libfontconfig-dev libxkbcommon-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# run this or ./bezier-gutton
./bezier
```

## How to play

* button 버전은 그냥 버튼과 슬라이더를 조절하면 된다.
* 키보드 버전은 기존 C++ 숏컷들을 그대로 사용 가능하며, 1부터 5까지의 숫자 버튼으로 biarc의 갯수를 조절할 수 있다.


## How to build

수업시간에 교수님께 다른 언어를 사용해도 된다는 허락을 받아 주 언어로 Rust를 사용하였다.

빌드가 안 된다면 다음 계정으로 메일을 보내주길 바란다. [hoyoung.jhoo@sf.snu.ac.kr](hoyoung.jhoo@sf.snu.ac.kr)

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

고급 라이브러리를 사용해서 자동으로 그려냈다는 의혹을 방지하기 위해 간단히 Rust 코드를 설명한다.

간편한 cross-platform 폰트 작업 및 UI, 윈도우를 관리하기 위해 iced 라이브러리를 사용하였으며, 그 안에서 Canvas의 기능을 주로 사용하였다.

Canvas는 HTML5 canvas의 기능과 비슷한 기능을 가졌으며, 기본적으로 `move_to`와 `line_to` method 등을 사용해 선을 그릴 수 있다.

Bezier curve를 그려내는 부분은 `biarc.rs`의 194-237 라인이며, 기존 C++의 코드를 그대로 가져왔다. Control point를 성정하는 부분은 같은 파일 146-154 라인에서 마우스 이벤트를 적절히 처리하여 설정한다.

```rust
// biarc.rs [194:237]
pub struct BezierCurve {
    control_pts: [Point; 4], // control points
}

impl BezierCurve {
    fn draw(&self, frame: &mut Frame, is_dotted: bool) {
        // generate curve
        let curve = Path::new(|p| {
            let mut point = Point::default();
            let mut dot_start = true;

            // move points
            p.move_to(self.control_pts[0]);

            for i in 1..=RESOLUTION {
                let t = (i as f32) / (RESOLUTION as f32);
                self.cubic_curve_to(&mut point, t);

                if is_dotted {
                    if dot_start {
                        p.line_to(point);
                    } else {
                        p.move_to(point);
                    }
                    dot_start = !dot_start;
                } else {
                    p.line_to(point);
                }
            }
        });

        // stroke line
        frame.stroke(&curve, Stroke::default().with_width(1.2));
    }

    fn cubic_curve_to(&self, point: &mut Point, t: f32) -> () {
        let t_inv = 1.0 - t;
        // ...
    }
}
```

이 라인 밑의 `build_biarc` 함수에서 BezierCurve struct를 바탕으로 Biarc의 컨트롤 포인트들을 세팅한다. 함수는 인자로 받은 `arcs` 벡터에 `2^num_biarc`개 만큼의 biarc를 집어넣는다.

`Biarc` struct는 두 원호의 중점과 반지름, 그리고 두 원호의 시작각도와 중간각도, 끝각도를 저장하고 있다. 원호를 그리는 방법은 `draw` 및 `draw_arc` 메소드에서 볼 수 있다.





