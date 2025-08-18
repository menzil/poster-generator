<template>
  <canvas id="posterCanvas" canvas-id="posterCanvas" :style="{ width: width + 'px', height: height + 'px' }" class="poster-canvas" />
</template>

<script>
export default {
  name: "Poster",
  props: {
    // 画布宽度
    width: {
      type: Number,
      default: 750,
    },
    // 画布高度
    height: {
      type: Number,
      default: 600, // 5:4比例，750 × 600
    },
    // 背景色
    backgroundColor: {
      type: String,
      default: "#ffffff",
    },
  },
  data() {
    return {
      ctx: null,
      elements: [], // 存储所有需要绘制的元素
      isDrawing: false,
    };
  },
  mounted() {
    this.$nextTick(() => {
      this.initCanvas();
    });
  },
  methods: {
    initCanvas() {
      try {
        this.ctx = uni.createCanvasContext("posterCanvas", this);
        console.log("Canvas初始化成功");
      } catch (error) {
        console.error("Canvas初始化失败:", error);
      }
    },

    // 加载图片
    loadImage(src) {
      return new Promise((resolve, reject) => {
        //console.log('加载图片:', src);

        // 检查是否为base64格式图片
        if (typeof src === "string" && (src.startsWith("data:image/") || /^[A-Za-z0-9+/=]+$/.test(src))) {
          console.log("检测到base64图片，开始下载");
          // 将base64转换为临时文件
          const base64 = src.startsWith("data:image/") ? src.split(",")[1] : src;
          const fileManager = uni.getFileSystemManager();
          const tempFilePath = `${uni.env.USER_DATA_PATH}/temp_${Date.now()}.png`;

          fileManager.writeFile({
            filePath: tempFilePath,
            data: base64,
            encoding: "base64",
            success: () => {
              console.log("base64图片写入成功:", tempFilePath);
              resolve(tempFilePath);
            },
            fail: (err) => {
              console.error("base64图片写入失败:", err);
              reject(err);
            },
          });
          return;
        }

        uni.getImageInfo({
          src: src,
          success: (image) => {
            console.log("图片加载成功:", src);
            resolve(image.path);
          },
          fail: (err) => {
            console.error("图片加载失败:", src, err);
            reject(err);
          },
        });
      });
    },

    // 清空元素
    clear() {
      this.elements = [];
      return this;
    },

    // 一次性设置所有元素
    setElements(elements = []) {
      if (!Array.isArray(elements)) {
        console.error("setElements参数必须是数组");
        return this;
      }

      // 清空现有元素
      this.clear();

      // 处理背景元素(如果有)
      const backgroundElement = elements.find((el) => el.type === "background");
      if (backgroundElement) {
        this.addBackground(backgroundElement);
      }

      // 添加其他元素
      elements
        .filter((el) => el.type !== "background")
        .forEach((element) => {
          if (element.type === "image") {
            this.addImage(element);
          } else if (element.type === "text") {
            this.addText(element);
          }
        });

      return this;
    },

    // 添加背景
    addBackground(options) {
      this.elements.push({
        type: "background",
        image: options.image || "",
        color: options.color || this.backgroundColor,
        radius: options.radius || 0,
      });
      return this;
    },

    // 添加图片元素
    addImage(options) {
      this.elements.push({
        type: "image",
        src: options.src,
        x: options.x || 0,
        y: options.y || 0,
        width: options.width || 200,
        height: options.height || 200,
        radius: options.radius || 0,
        zIndex: options.zIndex || 0,
        objectFit: options.objectFit || "cover",
      });
      return this;
    },

    // 添加文本元素
    addText(options) {
      this.elements.push({
        type: "text",
        text: options.text || "",
        x: options.x || 0,
        y: options.y || 0,
        fontSize: options.fontSize || 28,
        color: options.color || "#333333",
        align: options.align || "left",
        maxWidth: options.maxWidth,
        lineHeight: options.lineHeight || 1.5,
        maxLines: options.maxLines,
        zIndex: options.zIndex || 0,
        bold: options.bold || false,
        prefix: options.prefix || "", // 前缀，例如价格的¥符号
        backgroundColor: options.backgroundColor || "", // 文字背景色
        padding: options.padding || 0, // 文字内边距
        borderRadius: options.borderRadius || 0, // 背景圆角
        width: options.width, // 指定背景宽度，不指定则自动计算
        height: options.height, // 指定背景高度，不指定则自动计算
      });
      return this;
    },

    // 绘制圆角矩形
    drawRoundRect(x, y, width, height, radius) {
      if (!radius) {
        this.ctx.rect(x, y, width, height);
        return;
      }

      // 处理数组形式的radius，允许分别设置四个角
      let radiusTopLeft, radiusTopRight, radiusBottomRight, radiusBottomLeft;

      if (Array.isArray(radius)) {
        // 如果是数组，按顺序应用到四个角: 左上, 右上, 右下, 左下
        [radiusTopLeft, radiusTopRight, radiusBottomRight, radiusBottomLeft] = radius;
        // 如果数组不足4个值，对缺失的值使用0
        radiusTopLeft = radiusTopLeft || 0;
        radiusTopRight = radiusTopRight || 0;
        radiusBottomRight = radiusBottomRight || 0;
        radiusBottomLeft = radiusBottomLeft || 0;
      } else {
        // 单个数值时，所有角使用相同的值
        radiusTopLeft = radiusTopRight = radiusBottomRight = radiusBottomLeft = radius;
      }

      this.ctx.beginPath();
      // 从左上角开始，顺时针绘制
      this.ctx.moveTo(x + radiusTopLeft, y);
      this.ctx.lineTo(x + width - radiusTopRight, y);
      if (radiusTopRight > 0) {
        this.ctx.arc(x + width - radiusTopRight, y + radiusTopRight, radiusTopRight, Math.PI * 1.5, Math.PI * 2);
      }
      this.ctx.lineTo(x + width, y + height - radiusBottomRight);
      if (radiusBottomRight > 0) {
        this.ctx.arc(x + width - radiusBottomRight, y + height - radiusBottomRight, radiusBottomRight, 0, Math.PI * 0.5);
      }
      this.ctx.lineTo(x + radiusBottomLeft, y + height);
      if (radiusBottomLeft > 0) {
        this.ctx.arc(x + radiusBottomLeft, y + height - radiusBottomLeft, radiusBottomLeft, Math.PI * 0.5, Math.PI);
      }
      this.ctx.lineTo(x, y + radiusTopLeft);
      if (radiusTopLeft > 0) {
        this.ctx.arc(x + radiusTopLeft, y + radiusTopLeft, radiusTopLeft, Math.PI, Math.PI * 1.5);
      }
      this.ctx.closePath();
    },

    // 绘制文本（支持多行）
    drawText(element) {
      const {
        text,
        x,
        y,
        fontSize = 28,
        color = "#333333",
        align = "left",
        maxWidth,
        lineHeight = 1.5,
        maxLines,
        bold,
        prefix,
        backgroundColor,
        padding = 0,
        borderRadius = 0,
        width,
        height,
      } = element;

      this.ctx.setFontSize(fontSize);
      this.ctx.setFillStyle(color);
      this.ctx.setTextAlign(align);

      // 处理加粗
      if (bold) {
        this.ctx.font = `bold ${fontSize}px sans-serif`;
      }

      // 完整文本内容（带前缀）
      const fullText = (prefix || "") + text;

      // 绘制背景（如果有）
      if (backgroundColor) {
        let bgX = x;
        let bgWidth;
        let bgHeight;

        // 根据对齐方式调整背景位置
        if (align === "center") {
          // 计算文本宽度
          const textWidth = width || this.ctx.measureText(fullText).width + padding * 2;
          bgX = x - textWidth / 2;
          bgWidth = textWidth;
        } else if (align === "right") {
          const textWidth = width || this.ctx.measureText(fullText).width + padding * 2;
          bgX = x - textWidth;
          bgWidth = textWidth;
        } else {
          // 默认左对齐
          bgWidth = width || (maxWidth || this.ctx.measureText(fullText).width) + padding * 2;
        }

        // 计算高度
        bgHeight = height || fontSize + padding * 2;

        // 绘制圆角背景
        this.ctx.save();
        this.ctx.setFillStyle(backgroundColor);

        if (borderRadius && (Array.isArray(borderRadius) ? Math.max(...borderRadius) > 0 : borderRadius > 0)) {
          this.drawRoundRect(bgX, y - fontSize - padding, bgWidth, bgHeight, borderRadius);
          this.ctx.fill();
        } else {
          this.ctx.fillRect(bgX, y - fontSize - padding, bgWidth, bgHeight);
        }
        this.ctx.restore();
      }

      if (!maxWidth) {
        // 单行文本
        this.ctx.fillText(fullText, x, y);
        return;
      }

      // 多行文本
      const chars = fullText.split("");
      let line = "";
      let lines = [];
      const lineHeightPx = fontSize * lineHeight;

      for (let i = 0; i < chars.length; i++) {
        const testLine = line + chars[i];
        const testWidth = this.ctx.measureText(testLine).width;

        if (testWidth > maxWidth && i > 0) {
          lines.push(line);
          line = chars[i];
          if (maxLines && lines.length >= maxLines - 1) {
            // 达到最大行数
            break;
          }
        } else {
          line = testLine;
        }
      }

      // 添加最后一行
      if (line) {
        if (maxLines && lines.length >= maxLines) {
          // 达到最大行数，添加省略号
          line = line.substring(0, line.length - 3) + "...";
        }
        lines.push(line);
      }

      // 绘制所有行
      lines.forEach((line, index) => {
        const lineY = y + index * lineHeightPx;
        this.ctx.fillText(line, x, lineY);
      });
    },

    // 获取图片信息的方法
    getImageInfo(src) {
      return new Promise((resolve, reject) => {
        // 检查是否为base64格式图片
        if (typeof src === "string" && src.startsWith("data:image/")) {
          console.log("检测到base64图片，使用默认尺寸");
          // 对于base64图片，我们无法直接获取其尺寸
          // 返回一个默认值或者通过其他方式估算尺寸
          resolve({
            path: src,
            width: 500, // 默认宽度
            height: 500, // 默认高度
          });
          return;
        }

        uni.getImageInfo({
          src: src,
          success: (res) => {
            console.log("获取图片信息成功:", res.width, "x", res.height);
            resolve({
              path: res.path,
              width: res.width,
              height: res.height,
            });
          },
          fail: (err) => {
            console.error("获取图片信息失败:", err);
            reject(err);
          },
        });
      });
    },

    // 绘制圆角图片
    async drawImage(element) {
      const { src, x, y, width, height, radius, objectFit = "cover" } = element;

      try {
        // 加载图片
        const imagePath = await this.loadImage(src);
        // 获取图片原始尺寸信息
        const imageInfo = await this.getImageInfo(imagePath);
        const originalWidth = imageInfo.width;
        const originalHeight = imageInfo.height;

        console.log("原始图片尺寸:", originalWidth, "x", originalHeight, "目标尺寸:", width, "x", height);

        // 保存当前状态
        this.ctx.save();

        // 如果需要圆角，先创建裁剪区域
        if (radius && (Array.isArray(radius) ? Math.max(...radius) > 0 : radius > 0)) {
          this.drawRoundRect(x, y, width, height, radius);
          this.ctx.clip();
        }

        // 检查是否是base64图片
        const isBase64 = typeof imagePath === "string" && imagePath.startsWith("data:image/");

        if (isBase64) {
          // 对于base64图片，我们简化处理方式，直接绘制到指定区域
          console.log("绘制base64图片到:", x, y, width, height);
          this.ctx.drawImage(imagePath, x, y, width, height);
        } else if (objectFit === "cover") {
          // 计算裁剪参数，保持图片中心区域
          let sourceX = 0,
            sourceY = 0,
            sourceWidth = originalWidth,
            sourceHeight = originalHeight;

          // 计算原始图片和目标区域的宽高比
          const originalRatio = originalWidth / originalHeight;
          const targetRatio = width / height;

          if (originalRatio > targetRatio) {
            // 原图更宽，裁剪左右两边
            sourceWidth = originalHeight * targetRatio;
            sourceX = (originalWidth - sourceWidth) / 2;
          } else if (originalRatio < targetRatio) {
            // 原图更高，裁剪上下两边
            sourceHeight = originalWidth / targetRatio;
            sourceY = (originalHeight - sourceHeight) / 2;
          }

          // 使用9参数版本的drawImage，进行裁剪和缩放
          this.ctx.drawImage(
            imagePath,
            sourceX,
            sourceY,
            sourceWidth,
            sourceHeight, // 源图像裁剪参数
            x,
            y,
            width,
            height // 目标图像参数
          );

          console.log("使用cover模式绘制图片:", "源区域:", sourceX, sourceY, sourceWidth, sourceHeight, "目标区域:", x, y, width, height);
        } else if (objectFit === "contain") {
          // 保持原始比例，在目标区域内缩放
          let drawWidth = width,
            drawHeight = height;
          let drawX = x,
            drawY = y;

          const originalRatio = originalWidth / originalHeight;
          const targetRatio = width / height;

          if (originalRatio > targetRatio) {
            // 原图更宽，适应宽度
            drawHeight = width / originalRatio;
            drawY = y + (height - drawHeight) / 2;
          } else if (originalRatio < targetRatio) {
            // 原图更高，适应高度
            drawWidth = height * originalRatio;
            drawX = x + (width - drawWidth) / 2;
          }

          // 绘制图片
          this.ctx.drawImage(imagePath, drawX, drawY, drawWidth, drawHeight);

          console.log("使用contain模式绘制图片:", "目标区域:", drawX, drawY, drawWidth, drawHeight);
        } else {
          // 默认模式，直接拉伸
          this.ctx.drawImage(imagePath, x, y, width, height);
          console.log("使用stretch模式绘制图片");
        }

        // 恢复状态
        this.ctx.restore();
      } catch (error) {
        console.error("图片绘制失败:", error);
      }
    },

    // 生成海报
    async generatePoster() {
      if (this.isDrawing) {
        console.log("海报正在生成中，请稍后再试");
        return Promise.reject(new Error("海报正在生成中"));
      }

      return new Promise((resolve, reject) => {
        this.isDrawing = true;

        if (!this.ctx) {
          console.error("Canvas上下文未找到，重新初始化");
          this.initCanvas();
          setTimeout(() => {
            this.isDrawing = false;
            // 递归调用并返回promise
            resolve(this.generatePoster());
          }, 300);
          return;
        }

        if (!this.elements.length) {
          console.warn("没有添加任何元素");
          this.$emit("error", new Error("没有添加任何元素"));
          this.isDrawing = false;
          reject(new Error("没有添加任何元素"));
          return;
        }

        console.log(`开始生成海报，元素数量: ${this.elements.length}`);

        // 继续绘制元素的函数
        const continueDrawing = async () => {
          try {
            // 按照zIndex排序其他元素
            const otherElements = this.elements.filter((el) => el.type !== "background").sort((a, b) => a.zIndex - b.zIndex);

            // 绘制所有元素
            for (const element of otherElements) {
              switch (element.type) {
                case "image":
                  await this.drawImage(element);
                  break;

                case "text":
                  this.drawText(element);
                  break;
              }
            }

            // 完成绘制
            console.log("绘制完成，准备输出");
            this.ctx.draw(false, () => {
              console.log("Canvas绘制完成，准备生成图片");
              setTimeout(() => {
                uni.canvasToTempFilePath(
                  {
                    canvasId: "posterCanvas",
                    success: (res) => {
                      console.log("生成图片成功:", res.tempFilePath);
                      this.$emit("generated", res.tempFilePath);
                      this.isDrawing = false;
                      resolve(res.tempFilePath); // 成功时resolve图片路径
                    },
                    fail: (err) => {
                      console.error("生成图片失败:", err);
                      this.$emit("error", err);
                      this.isDrawing = false;
                      reject(err); // 失败时reject错误
                    },
                  },
                  this
                );
              }, 300); // 延迟确保绘制完成
            });
          } catch (error) {
            console.error("绘制元素失败:", error);
            this.$emit("error", error);
            this.isDrawing = false;
            reject(error);
          }
        };

        try {
          // 清空画布
          this.ctx.clearRect(0, 0, this.width, this.height);

          // 先绘制背景
          const backgroundElements = this.elements.filter((el) => el.type === "background");
          if (backgroundElements.length > 0) {
            const bg = backgroundElements[0];

            // 如果有圆角设置
            const radius = bg.radius;
            const hasRadius = radius && (Array.isArray(radius) ? Math.max(...radius) > 0 : radius > 0);

            if (bg.image) {
              this.loadImage(bg.image)
                .then((bgPath) => {
                  if (hasRadius) {
                    // 如果需要圆角，先创建裁剪区域
                    this.ctx.save();
                    this.drawRoundRect(0, 0, this.width, this.height, radius);
                    this.ctx.clip();
                    this.ctx.drawImage(bgPath, 0, 0, this.width, this.height);
                    this.ctx.restore();
                  } else {
                    this.ctx.drawImage(bgPath, 0, 0, this.width, this.height);
                  }
                  continueDrawing();
                })
                .catch((error) => {
                  console.error("背景图加载失败:", error);
                  // 使用背景色
                  this.ctx.setFillStyle(bg.color || this.backgroundColor);
                  if (hasRadius) {
                    this.ctx.save();
                    this.drawRoundRect(0, 0, this.width, this.height, radius);
                    this.ctx.fill();
                    this.ctx.restore();
                  } else {
                    this.ctx.fillRect(0, 0, this.width, this.height);
                  }
                  continueDrawing();
                });
            } else {
              // 使用背景色
              this.ctx.setFillStyle(bg.color || this.backgroundColor);
              if (hasRadius) {
                this.ctx.save();
                this.drawRoundRect(0, 0, this.width, this.height, radius);
                this.ctx.fill();
                this.ctx.restore();
              } else {
                this.ctx.fillRect(0, 0, this.width, this.height);
              }
              continueDrawing();
            }
          } else {
            // 默认背景色
            this.ctx.setFillStyle(this.backgroundColor);
            this.ctx.fillRect(0, 0, this.width, this.height);
            continueDrawing();
          }
        } catch (error) {
          console.error("生成海报失败:", error);
          this.$emit("error", error);
          this.isDrawing = false;
          reject(error);
        }
      });
    },
  },
};
</script>

<style lang="scss">
.poster-canvas {
  position: fixed;
  left: -9999px;
  top: -9999px;
}
</style>
