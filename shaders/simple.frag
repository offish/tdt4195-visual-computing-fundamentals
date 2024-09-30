#version 430 core

in vec4 fragmentColor; // color from vert (interpolated color)
in vec3 normal;
out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    vec3 myColor = vec3(1, 1, 1) * max(0, dot(normal, -lightDirection));

    // set color
    color = vec4(myColor, 1);
    // color = fragmentColor;
}
