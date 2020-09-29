//! 自动化集成测试

use lnrust::autotests;

#[test]
fn tst_run() {
    // 测试autotests模块的pub函数
    assert!(autotests::run(), true);
}
