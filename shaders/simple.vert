#version 430 core

layout(location = 0) in vec3 position; // xyz
layout(location = 1) in vec4 vertexColor; // RGBA

out vec4 fragmentColor; // output to fragment shader

void main()
{
    // give color to fragment shader
    fragmentColor = vertexColor;
    
    // set position of current vertex
    gl_Position = vec4(position, 1.0f);
}
