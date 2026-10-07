// Reading a Minecraft skin image to draw the player's head.

export interface Skin {
  /** `data:image/png;base64,…` URL of the whole skin. */
  url: string;
  /** Whether to draw the hat layer over the face. */
  showHat: boolean;
}

/**
 * Old skins (64x32, before Minecraft 1.8) often fill the hat area with an
 * opaque color, usually black. Like the game, we ignore the hat layer of an
 * old skin when it is fully opaque, otherwise the face would be hidden.
 */
export async function loadSkin(url: string): Promise<Skin> {
  const image = new Image();
  image.src = url;
  await image.decode();

  const isLegacy = image.height === 32;
  if (!isLegacy) return { url, showHat: true };

  const canvas = document.createElement("canvas");
  canvas.width = image.width;
  canvas.height = image.height;
  const context = canvas.getContext("2d");
  if (!context) return { url, showHat: false };
  context.drawImage(image, 0, 0);

  // Hat front: 8x8 square at (40, 8). Every 4th byte is the alpha channel.
  const pixels = context.getImageData(40, 8, 8, 8).data;
  let fullyOpaque = true;
  for (let i = 3; i < pixels.length; i += 4) {
    if (pixels[i] < 255) fullyOpaque = false;
  }
  return { url, showHat: !fullyOpaque };
}
