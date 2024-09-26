TARGET = robustmq
BUILD_FOLD = ./build
VERSION := $(shell cat version.ini)
PACKAGE_FOLD_NAME = ${TARGET}-${VERSION}


release:
	# 创建 build 目录
	mkdir -p ${BUILD_FOLD}
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/libs
	mkdir -p $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/config
	# 编译 release 包
	cargo build --release

	# 拷贝 bin 目录下的脚本、config 中的配置文件、编译成功的可执行文件
	cp -rf target/release/placement-center $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/libs
	cp -rf bin/* $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin
	cp -rf config/* $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/config
	chmod -R 777 $(BUILD_FOLD)/${PACKAGE_FOLD_NAME}/bin/*

	# 将目录打包成 .tar.gz 文件
	cd $(BUILD_FOLD) && tar zcvf ${PACKAGE_FOLD_NAME}.tar.gz ${PACKAGE_FOLD_NAME} && rm -rf ${PACKAGE_FOLD_NAME}
	echo "build release package success. ${PACKAGE_FOLD_NAME}.tar.gz"


test:
	sh ./scripts/integration_test.sh
clean:
	cargo clean
	rm -rf build
