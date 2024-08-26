import { instantiate } from "./mod.ts";

const { resize_image } = await instantiate();

Deno.test("test: resize_image", () => {
  let image = Deno.readFileSync("test/sample_1.png");
  image = new Uint8Array(image);

  const png = resize_image(image, 300, 150, "png");
  const webp = resize_image(image, 300, 150, "png");

  Deno.writeFileSync("test/sample_1_ts_resized.png", new Uint8Array(png));
  Deno.writeFileSync("test/sample_1_ts_resized.webp", new Uint8Array(webp));
});
