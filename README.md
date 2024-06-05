# rust-study
## 배경 및 목적
- Rust 기반 개발툴[1]이 JavaScript[2] 기반 개발툴보다 빠르다고 한다.
- 그래서 Rust 기반 개발툴을 직접 만들어보면서 DX[3]를 향상 시킬 수 있는 개발툴을 만드는 게 목적이다.

[1] [rspack](https://www.rspack.dev/), [swc](https://swc.rs/)가 있음. <br />
[2] JavaScript Runtime은 [Node.js](https://nodejs.org/en)와 [Bun](https://bun.sh/)이 있으므로 Node.js라고 이제 안써야 할 듯.<br />
[3] DX: Developer eXperience - [관련 포스트](https://toss.tech/article/tech-writer-2)

## 학습 자료
- [x] Rust 학습 | MS : https://learn.microsoft.com/ko-kr/training/paths/rust-first-steps/
- [ ] Rust 문법 : https://rinthel.github.io/rust-lang-book-ko/foreword.html
- [ ] CookBook : https://rust-lang-nursery.github.io/rust-cookbook/

## Rust 세계 지식 메모
- 러스트는 모든 프로그램의 스코프에 [prelude](https://doc.rust-lang.org/std/prelude/index.html) 내의 타입들을 가져옴
  - prelude에 없다면 `use` 문을 활용해서 명시적으로 가져와야함
- `Cargo.toml` 을 변경하지 않으면 `cargo build`에서도 아무 일도 발생하지 않음
- 명시적으로 버전 업데이트를 하지 않으면 `Cargo.lock`을 참고한다.
- `extern crate <package>` 외부에 의존하는 크레이트가 있음을 명시함
- `cargo doc --open`을 하면 모든 의존 패키지들의 문서를 빌드해서 브라우저에 표시해줌.