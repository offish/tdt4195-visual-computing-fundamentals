#version 430 core

in vec4 fragmentColor; // color from vert (interpolated color)
out vec4 color;

void main()
{
    // set color
    color = fragmentColor;
}
