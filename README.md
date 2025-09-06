# rust-practice

튜토리얼 문서: [The Rust Programming Language](https://doc.rust-kr.org/title-page.html)

API 가이드라인: [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)

## 스터디 노트

#### 환경 도구
* rustup

  * Rust 툴체인 버전 관리 도구. Rust 설치/업데이트/버전 관리를 담당
  * rustc, std, cargo 얘가 다 관리함

* cargo

  * Rust 빌드/패키지 관리 도구
  * 프로젝트 생성, 빌드, 실행, 의존성 관리, 테스트, 문서화 담당

#### 로컬에서 docs 띄우기
* 프로젝트에 설정된 의존성 패키지들 모아보기

  ```
  $ cargo doc --open
  ```

* rust docs 띄우기
  ```
  // 문서 인덱스
  $ rustup doc

  // 특정 모듈 문서
  $ rustup doc std::io::stdin
  ```

#### 챕터별 정리
* [Ch03: Basic](./ch03-basic/note.md)

* [Ch04: Ownership](./ch04-ownership/note.md)