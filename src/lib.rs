// lib.rs -- Aldaron's System Interface / OpenGL
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

extern crate libc;

use std::mem;
use std::ptr;
use std::ffi::CStr;

type Void = i8;

mod loader;
mod types;

use types::*;

#[derive(Copy, Clone)]
pub struct Texture(libc::c_uint);
pub struct Attribute(pub GLint); // Pub is for testing,

/// The OpenGL builder.
pub struct OpenGLBuilder {
	lib: loader::Lib,
	display: loader::Display,
}

impl OpenGLBuilder {
	/// Begin the building.
	pub fn new() -> Option<(OpenGLBuilder, i32)> {
		if let Some(lib) = loader::Lib::new() {
			let (mut display, visual_id) = lib.init();

			Some((OpenGLBuilder {
				lib,
				display,
			}, visual_id))
		} else {
			None
		}
	}

	/// Complete the building
	pub fn to_opengl(mut self, window: EGLNativeWindowType) -> OpenGL {
		self.lib.init2(&mut self.display, window);

		OpenGL {
			// FFI OpenGL Functions.
			clear: self.lib.load(b"glClear\0"),
			clear_color: self.lib.load(b"glClearColor\0"),
			disable: self.lib.load(b"glDisable\0"),
			enable: self.lib.load(b"glEnable\0"),
			get_error: self.lib.load(b"glGetError\0"),
			blend_func_separate:
				self.lib.load(b"glBlendFuncSeparate\0"),
			create_shader: self.lib.load(b"glCreateShader\0"),
			shader_source: self.lib.load(b"glShaderSource\0"),
			compile_shader: self.lib.load(b"glCompileShader\0"),
			create_program: self.lib.load(b"glCreateProgram\0"),
			attach_shader: self.lib.load(b"glAttachShader\0"),
			link_program: self.lib.load(b"glLinkProgram\0"),
			get_string: self.lib.load(b"glGetString\0"),
			uniform: self.lib.load(b"glGetUniformLocation\0"),
			gen_buffers: self.lib.load(b"glGenBuffers\0"),
			bind_buffer: self.lib.load(b"glBindBuffer\0"),
			buffer_data: self.lib.load(b"glBufferData\0"),
			attribute: self.lib.load(b"glGetAttribLocation\0"),
			get_shader: self.lib.load(b"glGetShaderiv\0"),
			info_log: self.lib.load(b"glGetShaderInfoLog\0"),
			draw_elements: self.lib.load(b"glDrawElements\0"),
			use_program: self.lib.load(b"glUseProgram\0"),
			uniform_mat4: self.lib.load(b"glUniformMatrix4fv\0"),
			bind_texture: self.lib.load(b"glBindTexture\0"),
			vertex_attrib: self.lib.load(b"glVertexAttribPointer\0"),
			gen_textures: self.lib.load(b"glGenTextures\0"),
			tex_params: self.lib.load(b"glTexParameteri\0"),
			tex_image: self.lib.load(b"glTexImage2D\0"),
			enable_vattrib: self.lib.load(b"glEnableVertexAttribArray\0"),
			viewport: self.lib.load(b"glViewport\0"),
			// Other
			lib: self.lib,
			display: self.display,
		}
	}
}

#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern "C" {
	fn glClear(a: GLbitfield) -> ();
}

/// The OpenGL context.
pub struct OpenGL {
	lib: loader::Lib,
	display: loader::Display,
	clear: unsafe extern "C" fn(GLbitfield) -> (),
	clear_color: unsafe extern "C" fn(GLfloat, GLfloat, GLfloat,
		GLfloat) -> (),
	disable: unsafe extern "C" fn(GLenum) -> (),
	enable: unsafe extern "C" fn(GLenum) -> (),
	get_error: unsafe extern "C" fn() -> GLenum,
	blend_func_separate: unsafe extern "C" fn(GLenum, GLenum, GLenum,
		GLenum) -> (),
	create_shader: unsafe extern "C" fn(GLenum) -> GLuint,
	shader_source: unsafe extern "C" fn(GLuint, GLsizei,
		*const *const GLchar, *const GLint) -> (),
	compile_shader: unsafe extern "C" fn(GLuint) -> (),
	create_program: unsafe extern "C" fn() -> GLuint,
	attach_shader: unsafe extern "C" fn(GLuint, GLuint) -> (),
	link_program: unsafe extern "C" fn(GLuint) -> (),
	get_string: unsafe extern "C" fn(GLenum) -> *const GLubyte,
	uniform: unsafe extern "C" fn(GLuint, *const GLchar) -> GLint,
	gen_buffers: unsafe extern "C" fn(GLsizei, *mut GLuint) -> (),
	bind_buffer: unsafe extern "C" fn(GLenum, GLuint) -> (),
	buffer_data: unsafe extern "C" fn(GLenum, GLsizeiptr,
		*const libc::c_void, GLenum) -> (),
	attribute: unsafe extern "C" fn(GLuint, *const GLchar) -> GLint,
	get_shader: unsafe extern "C" fn(GLuint, GLenum, *mut GLint) -> (),
	info_log: unsafe extern "C" fn(GLuint, GLsizei, *mut GLsizei,
		*mut GLchar) -> (),
	draw_elements: unsafe extern "C" fn(GLenum, GLsizei, GLenum,
		*const libc::c_void) -> (),
	use_program: unsafe extern "C" fn(GLuint) -> (),
	uniform_mat4: unsafe extern "C" fn(GLint, GLsizei, GLboolean,
		*const GLfloat) -> (),
	bind_texture: unsafe extern "C" fn(GLenum, GLuint) -> (),
	vertex_attrib: unsafe extern "C" fn(GLuint, GLint, GLenum,
		GLboolean, GLsizei, *const libc::c_void) -> (),
	gen_textures: unsafe extern "C" fn(GLsizei, *mut GLuint) -> (),
	tex_params: unsafe extern "C" fn(GLenum, GLenum, GLint) -> (),
	tex_image: unsafe extern "C" fn(GLenum, GLint, GLint, GLsizei,
		GLsizei, GLint, GLenum, GLenum, *const libc::c_void) -> (),
	enable_vattrib: unsafe extern "C" fn(GLuint) -> (),
	viewport: unsafe extern "C" fn(GLint, GLint, GLsizei, GLsizei) -> (),
}

impl OpenGL {
	/// Clear the screen with a specific color.
	pub fn clear(&self) {
		// Clear Color & Depth
		unsafe {
			(self.clear)(0x00000100 | 0x00004000);
			self.error()
		}
		println!("LOG: glClear()");
	}

	/// Set the color for `clear`.
	pub fn color(&self, r: f32, g: f32, b: f32) {
		unsafe {
			(self.clear_color)(r, g, b, 1.0);
			self.error()
		}
		println!("LOG: glClearColor()");
	}

	/// Update the screen
	pub fn update(&self) {
		// Swap Display
		self.display.swap();
		println!("LOG: Swap()");
	}

	/// Enable something
	pub fn enable(&self, what: u32) {
		unsafe {
			(self.enable)(what);
			self.error()
		}
		println!("LOG: glEnable()");
	}

	/// Disable something
	pub fn disable(&self, what: u32) {
		unsafe {
			(self.disable)(what);
			self.error()
		}
		println!("LOG: glDisable()");
	}

	/// Configure blending
	pub fn blend(&self) {
		const GL_SRC_ALPHA : u32 = 0x0302;
		const GL_ONE_MINUS_SRC_ALPHA : u32 = 0x0303;
		const GL_DST_ALPHA : u32 = 0x0304;

		unsafe {
			(self.blend_func_separate)(
				GL_SRC_ALPHA,
				GL_ONE_MINUS_SRC_ALPHA,
				GL_SRC_ALPHA,
				GL_DST_ALPHA
			);
			self.error()
		}
		println!("LOG: glBlendFuncSeparate()");
	}

	/// Load a shader program
	pub fn shader(&self, vertex: &[u8], fragment: &[u8]) -> u32 {
		// Last character in slices needs to null for it to be safe.
		assert_eq!(vertex[vertex.len() -1], b'\0');
		assert_eq!(fragment[fragment.len() -1], b'\0');

		let program;

		unsafe {
			self.error();
			let v_shader = (self.create_shader)(0x8B31/*vertex*/);
			self.error();
			(self.shader_source)(v_shader, 1,
				[vertex.as_ptr() as *const _].as_ptr(), ptr::null());
			self.error();
			(self.compile_shader)(v_shader);
			self.error();
			//		if cfg!(debug) { // TODO
			// unsafe {
				let mut value = mem::uninitialized();

				(self.get_shader)(v_shader,
					0x8B81 /*GL_COMPILE_STATUS*/,
					&mut value);
				println!("E1");
				self.error();

				if value == 0 {
					let mut value = mem::uninitialized();
					(self.get_shader)(v_shader,
						0x8B84 /*GL_INFO_LOG_LENGTH*/,
						&mut value);

					self.error();
					let mut buffer : Vec<u8> =
						vec![mem::uninitialized();
							value as usize];
					(self.info_log)(v_shader,
						value as GLsizei,
						ptr::null_mut(),
						buffer.as_mut_ptr() as *mut _);
					self.error();

					panic!("Failed to compile: {}.",
						::std::str::from_utf8(
							buffer.as_slice())
							.unwrap());
				}
			// }
//		}

			let f_shader = (self.create_shader)(0x8B30/*fragment*/);
			self.error();
			(self.shader_source)(f_shader, 1,
				[fragment.as_ptr() as *const _].as_ptr(), ptr::null());
			self.error();
			(self.compile_shader)(f_shader);
			self.error();
			//		if cfg!(debug) { // TODO
			// unsafe {
				let mut value = mem::uninitialized();

				(self.get_shader)(f_shader,
					0x8B81 /*GL_COMPILE_STATUS*/,
					&mut value);
				self.error();

				if value == 0 {
					let mut value = mem::uninitialized();
					(self.get_shader)(f_shader,
						0x8B84 /*GL_INFO_LOG_LENGTH*/,
						&mut value);

					self.error();
					let mut buffer : Vec<u8> =
						vec![mem::uninitialized();
							value as usize];
					(self.info_log)(f_shader,
						value as GLsizei,
						ptr::null_mut(),
						buffer.as_mut_ptr() as *mut _);
				println!("E6");
					self.error();

					panic!("Failed to compile: {}.",
						::std::str::from_utf8(
							buffer.as_slice())
							.unwrap());
				}
			// }
//		}

			program = (self.create_program)();
			self.error();

			(self.attach_shader)(program, v_shader);
			self.error();
			(self.attach_shader)(program, f_shader);
			self.error();

			(self.link_program)(program);
			self.error();
		}

		println!("LOG: Make Shader()");

		program
	}

	/// Get uniform from a shader.
	pub fn uniform(&self, shader: u32, name: &[u8]) -> i32 {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');

		unsafe {
			let r = (self.uniform)(shader, name.as_ptr() as *const _);
			if r == -1 {
				panic!("Error No Uniform: {:?}",
					::std::str::from_utf8(name).unwrap());
			}
			self.error();
			r
		}
	}

	/// Get the attribute
	pub fn attribute(&self, shader: u32, name: &[u8]) -> Attribute {
		// Last character in slice needs to null for it to be safe.
		assert_eq!(name[name.len() -1], b'\0');

		let attrib = unsafe {
			let a = (self.attribute)(shader, name.as_ptr() as *const _);
			if a == -1 {
				panic!("Error No Attribute: {:?}",
					::std::str::from_utf8(name).unwrap());
			}
			self.error();

			(self.enable_vattrib)(a as u32);
			self.error();

			a
		};

		Attribute(attrib)
	}

	/// Create some new buffers
	pub fn new_buffers(&self, n: usize) -> Vec<u32> {
		unsafe {
			let mut buffers = vec![mem::uninitialized(); n];

			(self.gen_buffers)(n as i32, buffers.as_mut_ptr());
			self.error();

			println!("LOG: glGenBuffers(n {})", n);

			buffers
		}
	}

	/// Bind a buffer from `new_buffers()`
	pub fn bind_buffer(&self, is_index_buffer: bool, buffer: u32) {
		unsafe {
			(self.bind_buffer)(
				if is_index_buffer {
		println!("LOG: glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, buffer {})", buffer);
					0x8893/*GL_ELEMENT_ARRAY_BUFFER*/
				} else {
		println!("LOG: glBindBuffer(GL_ARRAY_BUFFER), buffer {}", buffer);
					0x8892/*GL_ARRAY_BUFFER*/
				}, buffer);
			self.error();
		}


	}

	/// Set the bound buffer's data
	pub fn set_buffer<T>(&self, is_index_buffer: bool, data: &[T]) {
		unsafe {
			(self.buffer_data)(
				if is_index_buffer {
		println!("LOG: glBufferData(GL_ELEMENT_ARRAY_BUFFER 0x8893)");
					0x8893/*GL_ELEMENT_ARRAY_BUFFER*/
				} else {
		println!("LOG: glBufferData(GL_ARRAY_BUFFER 0x8892)");
					0x8892/*GL_ARRAY_BUFFER*/
				}, (data.len() * mem::size_of::<T>()) as isize,
				data.as_ptr() as *const _,
				0x88E8/*GL_DYNAMIC_DRAW*/);
			self.error();
		}


	}

	// TODO: this actually unsafe because uniforms can only be accessed when
	// their program is in use.
	/// Use a program.
	pub fn use_program(&self, shader: u32) {
		unsafe {
			(self.use_program)(shader);
			self.error();
		}
		println!("LOG: glUseProgram(shader {})", shader);
	}

	/// Set a uniform to a Mat4
	pub fn set_mat4(&self, uniform: i32, mat4: &[f32; 16]) -> () {
		unsafe {
			// set transformation matrix
			(self.uniform_mat4)(uniform, 1, 0 /*bool: transpose*/,
				mat4.as_ptr());
			self.error();
		}
	}

	/// Draw the elements.
	pub fn draw_elements(&self, n_indices: u32) {
		unsafe {
			// draw
			(self.draw_elements)(0x0004 /*GL_TRIANGLES*/,
				n_indices as GLsizei,
				0x1405 /*GL_UNSIGNED_INT*/, ptr::null());
			self.error();
		}

		println!("LOG: glDrawElements(GL_TRIANGLES (0x0004), {}, GL_UNSIGNED_INT (0x1405), null)", n_indices);
	}

	/// Create a new texture.
	pub fn new_texture(&self) -> Texture {
		Texture(unsafe {
			let mut a = mem::uninitialized();

			(self.gen_textures)(1, &mut a);
			self.error();

			self.use_texture(&Texture(a));

			(self.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER,
				GL_NEAREST);
			self.error();
			(self.tex_params)(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER,
				GL_NEAREST);
			self.error();

			a
		})
	}

	/// Set the bound texture's pixels
	pub fn set_texture(&self, w: u32, h: u32, px: &[u32]) {
		unsafe {	
			(self.tex_image)(GL_TEXTURE_2D, 0, GL_RGBA as i32,
				w as i32, h as i32, 0, GL_RGBA,
				GL_UNSIGNED_BYTE, px.as_ptr() as *const _);
			self.error();
		}
	}

	/// Use a texture.
	pub fn use_texture(&self, texture: &Texture) {
		unsafe {
			(self.bind_texture)(GL_TEXTURE_2D, texture.0);
			self.error();
		}
	}

	/// Set vertex attribute to current buffer.
	pub fn vertex_attrib(&self, attrib: &Attribute) {
		unsafe {
			(self.vertex_attrib)(attrib.0 as GLuint, 4, GL_FLOAT, 0,
				0, ptr::null());
			self.error();
		}

		println!("LOG: glVertexAttribPointer({}, 4, GL_FLOAT {}, 0, 0, null)", attrib.0, GL_FLOAT);
	}

	/// Update the viewport.
	pub fn viewport(&self, w: u32, h: u32) {
		unsafe {
			(self.viewport)(0, 0, w as GLsizei, h as GLsizei);
			self.error();
		}

		println!("LOG: glViewport(w {}, h {})", w, h);
	}

	/// Print out OpenGL version information to the console.
	pub fn version(&self) {
/*		const GL_VENDOR : u32 = 0x1F00;
		const GL_RENDERER : u32 = 0x1F01;
		const GL_VERSION : u32 = 0x1F02;
//		const GL_EXTENSIONS : u32 = 0x1F03;

		let vendor = unsafe {
			CStr::from_ptr((self.get_string)(GL_VENDOR) as *const _)
				.to_str().unwrap().to_string()
		};
		let renderer = unsafe {
			CStr::from_ptr((self.get_string)(GL_RENDERER) as *const _)
				.to_str().unwrap().to_string()
		};
		let version = unsafe {
			CStr::from_ptr((self.get_string)(GL_VERSION) as *const _)
				.to_str().unwrap().to_string()
		};
//		let extensions = unsafe {
//			CStr::from_ptr((self.get_string)(GL_EXTENSIONS))
//				.to_str().unwrap().to_string()
//		};

		println!("Vendor: {}", vendor);
		println!("Renderer: {}", renderer);
		println!("Version: {}", version);
//		println!("Extensions: {}", extensions);*/
	}

	unsafe fn error(&self) {
		match (self.get_error)() {
			0 => return, // NO_ERROR
			0x0500 => panic!("OpenGL Error: Invalid enum"),
			0x0501 => panic!("OpenGL Error: Invalid value"),
			0x0502 => panic!("OpenGL Error: Invalid operation"),
			0x0503 => panic!("OpenGL Error: Stack overflow"),
			0x0504 => panic!("OpenGL Error: Stack underflow"),
			0x0505 => panic!("OpenGL Error: Out of memory"),
			_ => panic!("OpenGL Error: Unknown"),
		}
	}
}