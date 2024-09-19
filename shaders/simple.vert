#version 430 core

layout(location = 0) in vec3 position; // xyz
layout(location = 1) in vec4 vertexColor; // RGBA
// uniform layout(location = 2) float elapsed;
uniform layout(location = 2) mat4x4 transformMatrix;
out vec4 fragmentColor; // output to fragment shader

void main()
{
    // identity matrix
    // mat4x4 identityMatrix = mat4(1);

    // float a = 1.0f;
    // float b = 0.0f;
    // float c = 0.0f;
    // float d = 0.0f;
    // float e = 1.0f;
    // float f = 0.0f;

    // identityMatrix[0][0] = a;
    // identityMatrix[0][1] = b;
    // identityMatrix[0][3] = c;
    // identityMatrix[1][0] = d;
    // identityMatrix[1][1] = e;
    // identityMatrix[1][3] = f;

    mat4x4 translationMatrix = mat4(1);
    translationMatrix[2][2] = -100.0f;

    // give color to fragment shader
    fragmentColor = vertexColor;
    
    gl_Position = transformMatrix * translationMatrix * vec4(position, 1.0f);
    // gl_Position = identityMatrix * vec4(position, 1.0f);
}
