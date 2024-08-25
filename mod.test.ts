import { instantiate } from "./mod.js";

const { resize_image } = await instantiate();

Deno.test("test: resize_image", () => {
  let image = Deno.readFileSync("test/sample_1.png");
  image = new Uint8Array(image);

  const result = resize_image(image, 300, 150);

  Deno.writeFileSync("test/sample_1_ts_resized.png", new Uint8Array(result));
});
