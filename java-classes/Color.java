class Color
{
  private int red;
  private int green;
  private int blue;
  private int alpha;

  public Color(int red, int green, int blue, int alpha)
  {
    setRGBA(red, green, blue, alpha)
  }

  public Color(int red, int green, int blue)
  {
    setRGB(red, green, blue, alpha)
  }

  private bool isInRange(int x)
  {
    return 255 >= x && 0 <= x;
  }

  public void setRBGA(int red, int green, int blue, int alpha)
  {
    if (!(isInRange(red) && isInRange(green) && isInRange(blue) && isInRange(alpha)))
      throw new IllegalArgumentException("");
    this.red = red;
    this.green = green;
    this.blue = blue;
  }

  public void setRBG(int red, int green, int blue)
  {
    setRGBA(red, green, blue, 255);
  }
}
