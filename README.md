# book-of-changes（用于易经个人研究 ）


git删除本地commit未提交文件 
`git rm -r --cached src/test.txt`
# cargo test 使用相关知识点
###### 1.单元测试:单元测试文件位于项目src/下(位置固定)
###### 2.集成单元测试:集成单元测试位于项目根目录下tests/下,即与src目录同级（位置固定）
###### 3.忽略测试test函数属性 `#[ignore]` 
###### 4. 开启测试函数print输出  `cargo test  sum -- --nocapture`(cargo test命令默认是关闭了test里的print输出
)
###### 5. cargo test(默认测试所有测试函数) cargo test it_works （指定具体方法名） cargo test it（正则匹配）



