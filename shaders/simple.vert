#version 430 core

layout(location = 0) in vec3 position; // xyz
layout(location = 1) in vec4 vertexColor; // RGBA
layout(location = 2) in vec3 normalVector; // xyz
// uniform layout(location = 2) float elapsed;
uniform layout(location = 3) mat4x4 transformMatrix;

out vec4 fragmentColor; // output to fragment shader
out vec3 normal;

void main()
{
    mat4x4 translationMatrix = mat4(1);
    translationMatrix[1][1] = -1.0f;
    translationMatrix[2][2] = -1.0f;

    // pass values to fragment shader
    fragmentColor = vertexColor;
    normal = normalVector;

    gl_Position = transformMatrix * translationMatrix * vec4(position, 1.0f);
    // gl_Position = identityMatrix * vec4(position, 1.0f);
}
