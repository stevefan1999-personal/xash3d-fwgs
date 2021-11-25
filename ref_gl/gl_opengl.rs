#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type grasshdr_s;
    pub type pmtrace_s;
    pub type con_nprint_s;
    pub type engine_studio_api_s;
    pub type r_studio_interface_s;
    pub type physent_s;
    pub type client_textmessage_s;
    pub type screenfade_s;
    pub type world_static_s;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut pglGetError: Option<unsafe extern "C" fn() -> GLenum>;
    #[no_mangle]
    static mut pglGetString:
           Option<unsafe extern "C" fn(_: GLenum) -> *const GLubyte>;
    #[no_mangle]
    static mut pglAccum:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglAlphaFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLclampf) -> ()>;
    #[no_mangle]
    static mut pglArrayElement: Option<unsafe extern "C" fn(_: GLint) -> ()>;
    #[no_mangle]
    static mut pglBegin: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglBindTexture:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()>;
    #[no_mangle]
    static mut pglBlendFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglClear: Option<unsafe extern "C" fn(_: GLbitfield) -> ()>;
    #[no_mangle]
    static mut pglClearColor:
           Option<unsafe extern "C" fn(_: GLclampf, _: GLclampf, _: GLclampf,
                                       _: GLclampf) -> ()>;
    #[no_mangle]
    static mut pglClearDepth: Option<unsafe extern "C" fn(_: GLclampd) -> ()>;
    #[no_mangle]
    static mut pglClearStencil: Option<unsafe extern "C" fn(_: GLint) -> ()>;
    #[no_mangle]
    static mut pglIsEnabled:
           Option<unsafe extern "C" fn(_: GLenum) -> GLboolean>;
    #[no_mangle]
    static mut pglIsList:
           Option<unsafe extern "C" fn(_: GLuint) -> GLboolean>;
    #[no_mangle]
    static mut pglIsTexture:
           Option<unsafe extern "C" fn(_: GLuint) -> GLboolean>;
    #[no_mangle]
    static mut pglClipPlane:
           Option<unsafe extern "C" fn(_: GLenum, _: *const GLdouble) -> ()>;
    #[no_mangle]
    static mut pglColor3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglColor3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglColor3ub:
           Option<unsafe extern "C" fn(_: GLubyte, _: GLubyte, _: GLubyte)
                      -> ()>;
    #[no_mangle]
    static mut pglColor4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglColor4fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglColor4ub:
           Option<unsafe extern "C" fn(_: GLubyte, _: GLubyte, _: GLubyte,
                                       _: GLubyte) -> ()>;
    #[no_mangle]
    static mut pglColor4ubv:
           Option<unsafe extern "C" fn(_: *const GLubyte) -> ()>;
    #[no_mangle]
    static mut pglColorMask:
           Option<unsafe extern "C" fn(_: GLboolean, _: GLboolean,
                                       _: GLboolean, _: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglColorPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCopyTexImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLint) -> ()>;
    #[no_mangle]
    static mut pglCopyTexImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglCopyTexSubImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLsizei) -> ()>;
    #[no_mangle]
    static mut pglCopyTexSubImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLint,
                                       _: GLsizei, _: GLsizei) -> ()>;
    #[no_mangle]
    static mut pglCullFace: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDeleteTextures:
           Option<unsafe extern "C" fn(_: GLsizei, _: *const GLuint) -> ()>;
    #[no_mangle]
    static mut pglDepthFunc: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDepthMask: Option<unsafe extern "C" fn(_: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglDepthRange:
           Option<unsafe extern "C" fn(_: GLclampd, _: GLclampd) -> ()>;
    #[no_mangle]
    static mut pglDisable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDisableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDrawArrays:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLsizei)
                      -> ()>;
    #[no_mangle]
    static mut pglDrawBuffer: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglDrawElements:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDrawPixels:
           Option<unsafe extern "C" fn(_: GLsizei, _: GLsizei, _: GLenum,
                                       _: GLenum, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglEnable: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnableClientState:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglEnd: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFinish: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFlush: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglFogf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglFogfv:
           Option<unsafe extern "C" fn(_: GLenum, _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglFogi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglFrontFace: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglFrustum:
           Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble,
                                       _: GLdouble, _: GLdouble, _: GLdouble)
                      -> ()>;
    #[no_mangle]
    static mut pglGenTextures:
           Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()>;
    #[no_mangle]
    static mut pglGetBooleanv:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLboolean) -> ()>;
    #[no_mangle]
    static mut pglGetClipPlane:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLdouble) -> ()>;
    #[no_mangle]
    static mut pglGetDoublev:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLdouble) -> ()>;
    #[no_mangle]
    static mut pglGetFloatv:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLfloat) -> ()>;
    #[no_mangle]
    static mut pglGetIntegerv:
           Option<unsafe extern "C" fn(_: GLenum, _: *mut GLint) -> ()>;
    #[no_mangle]
    static mut pglGetTexEnviv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: *mut GLint)
                      -> ()>;
    #[no_mangle]
    static mut pglGetTexImage:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLenum, _: *mut libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglGetTexLevelParameterfv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: *mut GLfloat) -> ()>;
    #[no_mangle]
    static mut pglGetTexLevelParameteriv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: *mut GLint) -> ()>;
    #[no_mangle]
    static mut pglHint:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglIndexPointer:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglLineWidth: Option<unsafe extern "C" fn(_: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglLoadIdentity: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglLoadMatrixd:
           Option<unsafe extern "C" fn(_: *const GLdouble) -> ()>;
    #[no_mangle]
    static mut pglLoadMatrixf:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglMatrixMode: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglMultMatrixd:
           Option<unsafe extern "C" fn(_: *const GLdouble) -> ()>;
    #[no_mangle]
    static mut pglMultMatrixf:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglNormal3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglNormal3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglNormalPointer:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglOrtho:
           Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble,
                                       _: GLdouble, _: GLdouble, _: GLdouble)
                      -> ()>;
    #[no_mangle]
    static mut pglPixelStoref:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglPixelStorei:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglPointSize: Option<unsafe extern "C" fn(_: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglPolygonMode:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum) -> ()>;
    #[no_mangle]
    static mut pglPolygonOffset:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglPolygonStipple:
           Option<unsafe extern "C" fn(_: *const GLubyte) -> ()>;
    #[no_mangle]
    static mut pglPopAttrib: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglPopMatrix: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglPushAttrib:
           Option<unsafe extern "C" fn(_: GLbitfield) -> ()>;
    #[no_mangle]
    static mut pglPushMatrix: Option<unsafe extern "C" fn() -> ()>;
    #[no_mangle]
    static mut pglRasterPos2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglReadBuffer: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglReadPixels:
           Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLenum, _: GLenum,
                                       _: *mut libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglRotated:
           Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble,
                                       _: GLdouble) -> ()>;
    #[no_mangle]
    static mut pglRotatef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglScaled:
           Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble)
                      -> ()>;
    #[no_mangle]
    static mut pglScalef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglScissor:
           Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei) -> ()>;
    #[no_mangle]
    static mut pglShadeModel: Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglStencilFunc:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLuint) -> ()>;
    #[no_mangle]
    static mut pglStencilMask: Option<unsafe extern "C" fn(_: GLuint) -> ()>;
    #[no_mangle]
    static mut pglStencilOp:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum)
                      -> ()>;
    #[no_mangle]
    static mut pglTexCoord1f: Option<unsafe extern "C" fn(_: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord1fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord2fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexCoord3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord4f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoord4fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexCoordPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexEnvf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexEnvfv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum,
                                       _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexEnvi:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexGenf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexGenfv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum,
                                       _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexGeni:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLint, _: GLenum,
                                       _: GLenum, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglTexImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLsizei, _: GLint,
                                       _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexImage2DMultisample:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizei, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLboolean)
                      -> ()>;
    #[no_mangle]
    static mut pglTexParameterf:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglTexParameterfv:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum,
                                       _: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglTexParameteri:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLint) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage1D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage2D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLsizei, _: GLsizei,
                                       _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTranslated:
           Option<unsafe extern "C" fn(_: GLdouble, _: GLdouble, _: GLdouble)
                      -> ()>;
    #[no_mangle]
    static mut pglTranslatef:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglVertex2f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglVertex3f:
           Option<unsafe extern "C" fn(_: GLfloat, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglVertex3fv:
           Option<unsafe extern "C" fn(_: *const GLfloat) -> ()>;
    #[no_mangle]
    static mut pglVertexPointer:
           Option<unsafe extern "C" fn(_: GLint, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglViewport:
           Option<unsafe extern "C" fn(_: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei) -> ()>;
    #[no_mangle]
    static mut pglActiveTextureARB:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglClientActiveTextureARB:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglGetCompressedTexImage:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDrawRangeElements:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint, _: GLuint,
                                       _: GLsizei, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDrawRangeElementsEXT:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint, _: GLuint,
                                       _: GLsizei, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglMultiTexCoord1f:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglMultiTexCoord2f:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat)
                      -> ()>;
    #[no_mangle]
    static mut pglMultiTexCoord3f:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat,
                                       _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglMultiTexCoord4f:
           Option<unsafe extern "C" fn(_: GLenum, _: GLfloat, _: GLfloat,
                                       _: GLfloat, _: GLfloat) -> ()>;
    #[no_mangle]
    static mut pglActiveTexture:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglClientActiveTexture:
           Option<unsafe extern "C" fn(_: GLenum) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage3DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLsizei,
                                       _: GLint, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage2DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLint,
                                       _: GLsizei, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglCompressedTexImage1DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLint, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage3DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLsizei, _: GLenum,
                                       _: GLsizei, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage2DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLsizei, _: GLsizei,
                                       _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglCompressedTexSubImage1DARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLsizei, _: GLenum, _: GLsizei,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexImage3D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLenum,
                                       _: GLsizei, _: GLsizei, _: GLsizei,
                                       _: GLint, _: GLenum, _: GLenum,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglTexSubImage3D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLsizei,
                                       _: GLsizei, _: GLsizei, _: GLenum,
                                       _: GLenum, _: *const libc::c_void)
                      -> ()>;
    #[no_mangle]
    static mut pglCopyTexSubImage3D:
           Option<unsafe extern "C" fn(_: GLenum, _: GLint, _: GLint,
                                       _: GLint, _: GLint, _: GLint, _: GLint,
                                       _: GLsizei, _: GLsizei) -> ()>;
    #[no_mangle]
    static mut pglBindBufferARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLuint) -> ()>;
    #[no_mangle]
    static mut pglDeleteBuffersARB:
           Option<unsafe extern "C" fn(_: GLsizei, _: *const GLuint) -> ()>;
    #[no_mangle]
    static mut pglGenBuffersARB:
           Option<unsafe extern "C" fn(_: GLsizei, _: *mut GLuint) -> ()>;
    #[no_mangle]
    static mut pglIsBufferARB:
           Option<unsafe extern "C" fn(_: GLuint) -> GLboolean>;
    #[no_mangle]
    static mut pglMapBufferARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum)
                      -> *mut libc::c_void>;
    #[no_mangle]
    static mut pglUnmapBufferARB:
           Option<unsafe extern "C" fn(_: GLenum) -> GLboolean>;
    #[no_mangle]
    static mut pglBufferDataARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLsizeiptrARB,
                                       _: *const libc::c_void, _: GLenum)
                      -> ()>;
    #[no_mangle]
    static mut pglBufferSubDataARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLintptrARB,
                                       _: GLsizeiptrARB,
                                       _: *const libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglDebugMessageControlARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLenum,
                                       _: GLsizei, _: *const GLuint,
                                       _: GLboolean) -> ()>;
    #[no_mangle]
    static mut pglDebugMessageInsertARB:
           Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLuint,
                                       _: GLenum, _: GLsizei,
                                       _: *const libc::c_char) -> ()>;
    #[no_mangle]
    static mut pglDebugMessageCallbackARB:
           Option<unsafe extern "C" fn(_: GL_DEBUG_PROC_ARB,
                                       _: *mut libc::c_void) -> ()>;
    #[no_mangle]
    static mut pglGetDebugMessageLogARB:
           Option<unsafe extern "C" fn(_: GLuint, _: GLsizei, _: *mut GLenum,
                                       _: *mut GLenum, _: *mut GLuint,
                                       _: *mut GLuint, _: *mut GLsizei,
                                       _: *mut libc::c_char) -> GLuint>;
    #[no_mangle]
    fn GL_CleanupAllTextureUnits();
    #[no_mangle]
    fn GL_Cull(cull: GLenum);
    #[no_mangle]
    fn SCR_TimeRefresh_f();
    #[no_mangle]
    fn R_ClearDecals();
    #[no_mangle]
    fn R_InitImages();
    #[no_mangle]
    fn R_ShutdownImages();
    #[no_mangle]
    fn R_ClearScene();
    #[no_mangle]
    fn GL_InitRandomTable();
    #[no_mangle]
    fn R_SpriteInit();
    #[no_mangle]
    fn R_StudioInit();
    #[no_mangle]
    fn R_AliasInit();
    #[no_mangle]
    static mut gEngfuncs: ref_api_t;
    #[no_mangle]
    static mut gpGlobals: *mut ref_globals_t;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn Q_strstr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Q_stristr(string: *const libc::c_char, string2: *const libc::c_char)
     -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type uint = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec4_t = [vec_t; 4];
pub type rgba_t = [byte; 4];
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type fs_offset_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dllfunc_s {
    pub name: *const libc::c_char,
    pub func: *mut *mut libc::c_void,
}
pub type dllfunc_t = dllfunc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
pub type cvar_t = cvar_s;
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color24 {
    pub r: byte,
    pub g: byte,
    pub b: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorVec {
    pub r: libc::c_uint,
    pub g: libc::c_uint,
    pub b: libc::c_uint,
    pub a: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmodel_t {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: [libc::c_int; 4],
    pub visleafs: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
pub type modtype_t = libc::c_int;
pub const mod_studio: modtype_t = 3;
pub const mod_alias: modtype_t = 2;
pub const mod_sprite: modtype_t = 1;
pub const mod_brush: modtype_t = 0;
pub const mod_bad: modtype_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mclipnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_short; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_s {
    pub name: [libc::c_char; 16],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub gl_texturenum: libc::c_int,
    pub texturechain: *mut msurface_s,
    pub anim_total: libc::c_int,
    pub anim_min: libc::c_int,
    pub anim_max: libc::c_int,
    pub anim_next: *mut texture_s,
    pub alternate_anims: *mut texture_s,
    pub fb_texturenum: libc::c_ushort,
    pub dt_texturenum: libc::c_ushort,
    pub unused: [libc::c_uint; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub plane: *mut mplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub light_s: libc::c_int,
    pub light_t: libc::c_int,
    pub polys: *mut glpoly_t,
    pub texturechain: *mut msurface_s,
    pub texinfo: *mut mtexinfo_t,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub lightmaptexturenum: libc::c_int,
    pub styles: [byte; 4],
    pub cached_light: [libc::c_int; 4],
    pub info: *mut mextrasurf_t,
    pub samples: *mut color24,
    pub pdecals: *mut decal_t,
}
pub type decal_t = decal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decal_s {
    pub pnext: *mut decal_t,
    pub psurface: *mut msurface_t,
    pub dx: libc::c_float,
    pub dy: libc::c_float,
    pub scale: libc::c_float,
    pub texture: libc::c_short,
    pub flags: libc::c_short,
    pub entityIndex: libc::c_short,
    pub position: vec3_t,
    pub polys: *mut glpoly_t,
    pub reserved: [libc::c_int; 4],
}
pub type glpoly_t = glpoly_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glpoly_s {
    pub next: *mut glpoly_s,
    pub chain: *mut glpoly_s,
    pub numverts: libc::c_int,
    pub flags: libc::c_int,
    pub verts: [[libc::c_float; 7]; 4],
}
pub type msurface_t = msurface_s;
pub type mextrasurf_t = mextrasurf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mextrasurf_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub surf: *mut msurface_s,
    pub dlight_s: libc::c_int,
    pub dlight_t: libc::c_int,
    pub lightmapmins: [libc::c_short; 2],
    pub lightextents: [libc::c_short; 2],
    pub lmvecs: [[libc::c_float; 4]; 2],
    pub deluxemap: *mut color24,
    pub shadowmap: *mut byte,
    pub lightmapchain: *mut msurface_s,
    pub detailchain: *mut mextrasurf_s,
    pub bevel: *mut mfacebevel_t,
    pub lumachain: *mut mextrasurf_s,
    pub parent: *mut cl_entity_s,
    pub mirrortexturenum: libc::c_int,
    pub mirrormatrix: [[libc::c_float; 4]; 4],
    pub grass: *mut grasshdr_s,
    pub grasscount: libc::c_ushort,
    pub numverts: libc::c_ushort,
    pub firstvertex: libc::c_int,
    pub reserved: [libc::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_entity_s {
    pub index: libc::c_int,
    pub player: qboolean,
    pub baseline: entity_state_t,
    pub prevstate: entity_state_t,
    pub curstate: entity_state_t,
    pub current_position: libc::c_int,
    pub ph: [position_history_t; 64],
    pub mouth: mouth_t,
    pub latched: latchedvars_t,
    pub lastmove: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub attachment: [vec3_t; 4],
    pub trivial_accept: libc::c_int,
    pub model: *mut model_s,
    pub efrag: *mut efrag_s,
    pub topnode: *mut mnode_s,
    pub syncbase: libc::c_float,
    pub visframe: libc::c_int,
    pub cvFloorColor: colorVec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_t,
    pub children: [*mut mnode_s; 2],
    pub firstsurface: libc::c_ushort,
    pub numsurfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct efrag_s {
    pub leaf: *mut mleaf_s,
    pub leafnext: *mut efrag_s,
    pub entity: *mut cl_entity_s,
    pub entnext: *mut efrag_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mleaf_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub compressed_vis: *mut byte,
    pub efrags: *mut efrag_s,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub cluster: libc::c_int,
    pub ambient_sound_level: [byte; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub needload: qboolean,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub mempool: poolhandle_t,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub radius: libc::c_float,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut dmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut mplane_t,
    pub numleafs: libc::c_int,
    pub leafs: *mut mleaf_t,
    pub numvertexes: libc::c_int,
    pub vertexes: *mut mvertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut medge_t,
    pub numnodes: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numtexinfo: libc::c_int,
    pub texinfo: *mut mtexinfo_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub numsurfedges: libc::c_int,
    pub surfedges: *mut libc::c_int,
    pub numclipnodes: libc::c_int,
    pub clipnodes: *mut mclipnode_t,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub hulls: [hull_t; 4],
    pub numtextures: libc::c_int,
    pub textures: *mut *mut texture_t,
    pub visdata: *mut byte,
    pub lightdata: *mut color24,
    pub entities: *mut libc::c_char,
    pub cache: cache_user_t,
}
pub type cache_user_t = cache_user_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_user_s {
    pub data: *mut libc::c_void,
}
pub type texture_t = texture_s;
pub type hull_t = hull_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hull_s {
    pub clipnodes: *mut mclipnode_t,
    pub planes: *mut mplane_t,
    pub firstclipnode: libc::c_int,
    pub lastclipnode: libc::c_int,
    pub clip_mins: vec3_t,
    pub clip_maxs: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_t {
    pub vecs: [[libc::c_float; 4]; 2],
    pub faceinfo: *mut mfaceinfo_t,
    pub texture: *mut texture_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfaceinfo_t {
    pub landname: [libc::c_char; 16],
    pub texture_step: libc::c_ushort,
    pub max_extent: libc::c_ushort,
    pub groupid: libc::c_short,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub reserved: [libc::c_int; 32],
}
pub type mnode_t = mnode_s;
pub type mleaf_t = mleaf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latchedvars_t {
    pub prevanimtime: libc::c_float,
    pub sequencetime: libc::c_float,
    pub prevseqblending: [byte; 2],
    pub prevorigin: vec3_t,
    pub prevangles: vec3_t,
    pub prevsequence: libc::c_int,
    pub prevframe: libc::c_float,
    pub prevcontroller: [byte; 4],
    pub prevblending: [byte; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouth_t {
    pub mouthopen: byte,
    pub sndcount: byte,
    pub sndavg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct position_history_t {
    pub animtime: libc::c_float,
    pub origin: vec3_t,
    pub angles: vec3_t,
}
pub type entity_state_t = entity_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_state_s {
    pub entityType: libc::c_int,
    pub number: libc::c_int,
    pub msg_time: libc::c_float,
    pub messagenum: libc::c_int,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub modelindex: libc::c_int,
    pub sequence: libc::c_int,
    pub frame: libc::c_float,
    pub colormap: libc::c_int,
    pub skin: libc::c_short,
    pub solid: libc::c_short,
    pub effects: libc::c_int,
    pub scale: libc::c_float,
    pub eflags: byte,
    pub rendermode: libc::c_int,
    pub renderamt: libc::c_int,
    pub rendercolor: color24,
    pub renderfx: libc::c_int,
    pub movetype: libc::c_int,
    pub animtime: libc::c_float,
    pub framerate: libc::c_float,
    pub body: libc::c_int,
    pub controller: [byte; 4],
    pub blending: [byte; 4],
    pub velocity: vec3_t,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub aiment: libc::c_int,
    pub owner: libc::c_int,
    pub friction: libc::c_float,
    pub gravity: libc::c_float,
    pub team: libc::c_int,
    pub playerclass: libc::c_int,
    pub health: libc::c_int,
    pub spectator: qboolean,
    pub weaponmodel: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub basevelocity: vec3_t,
    pub usehull: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub onground: libc::c_int,
    pub iStepLeft: libc::c_int,
    pub flFallVelocity: libc::c_float,
    pub fov: libc::c_float,
    pub weaponanim: libc::c_int,
    pub startpos: vec3_t,
    pub endpos: vec3_t,
    pub impacttime: libc::c_float,
    pub starttime: libc::c_float,
    pub iuser1: libc::c_int,
    pub iuser2: libc::c_int,
    pub iuser3: libc::c_int,
    pub iuser4: libc::c_int,
    pub fuser1: libc::c_float,
    pub fuser2: libc::c_float,
    pub fuser3: libc::c_float,
    pub fuser4: libc::c_float,
    pub vuser1: vec3_t,
    pub vuser2: vec3_t,
    pub vuser3: vec3_t,
    pub vuser4: vec3_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mfacebevel_t {
    pub edges: *mut mplane_t,
    pub numedges: libc::c_int,
    pub origin: vec3_t,
    pub radius: vec_t,
    pub contents: libc::c_int,
}
pub type model_t = model_s;
pub type resourcetype_t = libc::c_uint;
pub const t_world: resourcetype_t = 6;
pub const t_eventscript: resourcetype_t = 5;
pub const t_generic: resourcetype_t = 4;
pub const t_decal: resourcetype_t = 3;
pub const t_model: resourcetype_t = 2;
pub const t_skin: resourcetype_t = 1;
pub const t_sound: resourcetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct resource_s {
    pub szFileName: [libc::c_char; 64],
    pub type_0: resourcetype_t,
    pub nIndex: libc::c_int,
    pub nDownloadSize: libc::c_int,
    pub ucFlags: libc::c_uchar,
    pub rgucMD5_hash: [libc::c_uchar; 16],
    pub playernum: libc::c_uchar,
    pub rguc_reserved: [libc::c_uchar; 32],
    pub pNext: *mut resource_s,
    pub pPrev: *mut resource_s,
}
pub type resource_t = resource_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct customization_s {
    pub bInUse: qboolean,
    pub resource: resource_t,
    pub bTranslated: qboolean,
    pub nUserData1: libc::c_int,
    pub nUserData2: libc::c_int,
    pub pInfo: *mut libc::c_void,
    pub pBuffer: *mut libc::c_void,
    pub pNext: *mut customization_s,
}
pub type customization_t = customization_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_info_s {
    pub userid: libc::c_int,
    pub userinfo: [libc::c_char; 256],
    pub name: [libc::c_char; 32],
    pub spectator: libc::c_int,
    pub ping: libc::c_int,
    pub packet_loss: libc::c_int,
    pub model: [libc::c_char; 64],
    pub topcolor: libc::c_int,
    pub bottomcolor: libc::c_int,
    pub renderframe: libc::c_int,
    pub gaitsequence: libc::c_int,
    pub gaitframe: libc::c_float,
    pub gaityaw: libc::c_float,
    pub prevgaitorigin: vec3_t,
    pub customdata: customization_t,
    pub hashedcdkey: [libc::c_char; 16],
}
pub type player_info_t = player_info_s;
pub type cl_entity_t = cl_entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub pattern: [libc::c_char; 256],
    pub map: [libc::c_float; 256],
    pub length: libc::c_int,
    pub value: libc::c_float,
    pub interp: qboolean,
    pub time: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_s {
    pub origin: vec3_t,
    pub radius: libc::c_float,
    pub color: color24,
    pub die: libc::c_float,
    pub decay: libc::c_float,
    pub minlight: libc::c_float,
    pub key: libc::c_int,
    pub dark: qboolean,
}
pub type dlight_t = dlight_s;
pub type gl_context_type_t = libc::c_uint;
pub const CONTEXT_TYPE_GL_CORE: gl_context_type_t = 3;
pub const CONTEXT_TYPE_GLES_2_X: gl_context_type_t = 2;
pub const CONTEXT_TYPE_GLES_1_X: gl_context_type_t = 1;
pub const CONTEXT_TYPE_GL: gl_context_type_t = 0;
pub type gles_wrapper_t = libc::c_uint;
pub const GLES_WRAPPER_GL4ES: gles_wrapper_t = 3;
pub const GLES_WRAPPER_WES: gles_wrapper_t = 2;
pub const GLES_WRAPPER_NANOGL: gles_wrapper_t = 1;
pub const GLES_WRAPPER_NONE: gles_wrapper_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct modelstate_s {
    pub sequence: libc::c_short,
    pub frame: libc::c_short,
    pub blending: [byte; 2],
    pub controller: [byte; 4],
    pub poseparam: [byte; 16],
    pub body: byte,
    pub skin: byte,
    pub scale: libc::c_short,
}
pub type modelstate_t = modelstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decallist_s {
    pub position: vec3_t,
    pub name: [libc::c_char; 64],
    pub entityIndex: libc::c_short,
    pub depth: byte,
    pub flags: byte,
    pub scale: libc::c_float,
    pub impactPlaneNormal: vec3_t,
    pub studio_state: modelstate_t,
}
pub type decallist_t = decallist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_viewpass_s {
    pub viewport: [libc::c_int; 4],
    pub vieworigin: vec3_t,
    pub viewangles: vec3_t,
    pub viewentity: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_overview_s {
    pub origin: vec3_t,
    pub rotated: qboolean,
    pub xLeft: libc::c_float,
    pub xRight: libc::c_float,
    pub yTop: libc::c_float,
    pub yBottom: libc::c_float,
    pub zFar: libc::c_float,
    pub zNear: libc::c_float,
    pub flZoom: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiotex_s {
    pub name: [libc::c_char; 64],
    pub flags: uint32_t,
    pub width: int32_t,
    pub height: int32_t,
    pub index: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct render_interface_s {
    pub version: libc::c_int,
    pub GL_RenderFrame: Option<unsafe extern "C" fn(_: *const ref_viewpass_s)
                                   -> libc::c_int>,
    pub GL_BuildLightmaps: Option<unsafe extern "C" fn() -> ()>,
    pub GL_OrthoBounds: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                                    _: *const libc::c_float)
                                   -> ()>,
    pub R_CreateStudioDecalList: Option<unsafe extern "C" fn(_:
                                                                 *mut decallist_t,
                                                             _: libc::c_int)
                                            -> libc::c_int>,
    pub R_ClearStudioDecals: Option<unsafe extern "C" fn() -> ()>,
    pub R_SpeedsMessage: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                     _: size_t) -> qboolean>,
    pub Mod_ProcessUserData: Option<unsafe extern "C" fn(_: *mut model_s,
                                                         _: qboolean,
                                                         _: *const byte)
                                        -> ()>,
    pub R_ProcessEntData: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub Mod_GetCurrentVis: Option<unsafe extern "C" fn() -> *mut byte>,
    pub R_NewMap: Option<unsafe extern "C" fn() -> ()>,
    pub R_ClearScene: Option<unsafe extern "C" fn() -> ()>,
    pub CL_UpdateLatchedVars: Option<unsafe extern "C" fn(_: *mut cl_entity_s,
                                                          _: qboolean) -> ()>,
}
pub type render_interface_t = render_interface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpc_desc_s {
    pub format: libc::c_int,
    pub name: [libc::c_char; 16],
    pub glFormat: uint,
    pub bpp: libc::c_int,
}
pub type C2RustUnnamed = libc::c_uint;
pub const IL_OVERVIEW: C2RustUnnamed = 64;
pub const IL_LOAD_DECAL: C2RustUnnamed = 32;
pub const IL_DDS_HARDWARE: C2RustUnnamed = 16;
pub const IL_DONTFLIP_TGA: C2RustUnnamed = 8;
pub const IL_ALLOW_OVERWRITE: C2RustUnnamed = 4;
pub const IL_KEEP_8BIT: C2RustUnnamed = 2;
pub const IL_USE_LERPING: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgbdata_s {
    pub width: word,
    pub height: word,
    pub depth: word,
    pub type_0: uint,
    pub flags: uint,
    pub encode: word,
    pub numMips: byte,
    pub palette: *mut byte,
    pub buffer: *mut byte,
    pub fogParams: rgba_t,
    pub size: size_t,
}
pub type rgbdata_t = rgbdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct studiohdr_s {
    pub ident: int32_t,
    pub version: int32_t,
    pub name: [libc::c_char; 64],
    pub length: int32_t,
    pub eyeposition: vec3_t,
    pub min: vec3_t,
    pub max: vec3_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub flags: int32_t,
    pub numbones: int32_t,
    pub boneindex: int32_t,
    pub numbonecontrollers: int32_t,
    pub bonecontrollerindex: int32_t,
    pub numhitboxes: int32_t,
    pub hitboxindex: int32_t,
    pub numseq: int32_t,
    pub seqindex: int32_t,
    pub numseqgroups: int32_t,
    pub seqgroupindex: int32_t,
    pub numtextures: int32_t,
    pub textureindex: int32_t,
    pub texturedataindex: int32_t,
    pub numskinref: int32_t,
    pub numskinfamilies: int32_t,
    pub skinindex: int32_t,
    pub numbodyparts: int32_t,
    pub bodypartindex: int32_t,
    pub numattachments: int32_t,
    pub attachmentindex: int32_t,
    pub studiohdr2index: int32_t,
    pub unused: int32_t,
    pub unused2: int32_t,
    pub unused3: int32_t,
    pub numtransitions: int32_t,
    pub transitionindex: int32_t,
}
pub type studiohdr_t = studiohdr_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudiobone_s {
    pub name: [libc::c_char; 32],
    pub parent: int32_t,
    pub unused: int32_t,
    pub bonecontroller: [int32_t; 6],
    pub value: [vec_t; 6],
    pub scale: [vec_t; 6],
}
pub type mstudiobone_t = mstudiobone_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioevent_s {
    pub frame: int32_t,
    pub event: int32_t,
    pub unused: int32_t,
    pub options: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioseqdesc_s {
    pub label: [libc::c_char; 32],
    pub fps: vec_t,
    pub flags: int32_t,
    pub activity: int32_t,
    pub actweight: int32_t,
    pub numevents: int32_t,
    pub eventindex: int32_t,
    pub numframes: int32_t,
    pub weightlistindex: int32_t,
    pub iklockindex: int32_t,
    pub motiontype: int32_t,
    pub motionbone: int32_t,
    pub linearmovement: vec3_t,
    pub autolayerindex: int32_t,
    pub keyvalueindex: int32_t,
    pub bbmin: vec3_t,
    pub bbmax: vec3_t,
    pub numblends: int32_t,
    pub animindex: int32_t,
    pub blendtype: [int32_t; 2],
    pub blendstart: [vec_t; 2],
    pub blendend: [vec_t; 2],
    pub groupsize: [uint8_t; 2],
    pub numautolayers: uint8_t,
    pub numiklocks: uint8_t,
    pub seqgroup: int32_t,
    pub entrynode: int32_t,
    pub exitnode: int32_t,
    pub nodeflags: uint8_t,
    pub cycleposeindex: uint8_t,
    pub fadeintime: uint8_t,
    pub fadeouttime: uint8_t,
    pub animdescindex: int32_t,
}
pub type mstudioseqdesc_t = mstudioseqdesc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mstudioanim_s {
    pub offset: [uint16_t; 6],
}
pub type mstudioanim_t = mstudioanim_s;
pub type ptype_t = libc::c_uint;
pub const pt_clientcustom: ptype_t = 10;
pub const pt_vox_grav: ptype_t = 9;
pub const pt_vox_slowgrav: ptype_t = 8;
pub const pt_blob2: ptype_t = 7;
pub const pt_blob: ptype_t = 6;
pub const pt_explode2: ptype_t = 5;
pub const pt_explode: ptype_t = 4;
pub const pt_fire: ptype_t = 3;
pub const pt_slowgrav: ptype_t = 2;
pub const pt_grav: ptype_t = 1;
pub const pt_static: ptype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_s {
    pub org: vec3_t,
    pub color: libc::c_short,
    pub packedColor: libc::c_short,
    pub next: *mut particle_s,
    pub vel: vec3_t,
    pub ramp: libc::c_float,
    pub die: libc::c_float,
    pub type_0: ptype_t,
    pub deathfunc: Option<unsafe extern "C" fn(_: *mut particle_s) -> ()>,
    pub callback: Option<unsafe extern "C" fn(_: *mut particle_s,
                                              _: libc::c_float) -> ()>,
    pub context: libc::c_uchar,
}
pub type particle_t = particle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedface_t {
    pub surf: *mut msurface_t,
    pub cull: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_globals_s {
    pub developer: qboolean,
    pub time: libc::c_float,
    pub oldtime: libc::c_float,
    pub realtime: libc::c_double,
    pub frametime: libc::c_double,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fullScreen: qboolean,
    pub wideScreen: qboolean,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub vforward: vec3_t,
    pub vright: vec3_t,
    pub vup: vec3_t,
    pub draw_surfaces: *mut sortedface_t,
    pub max_surfaces: libc::c_int,
    pub visbytes: size_t,
    pub desktopBitsPixel: libc::c_int,
}
pub type ref_globals_t = ref_globals_s;
pub type C2RustUnnamed_0 = libc::c_int;
pub const MAX_TEXTURE_UNITS: C2RustUnnamed_0 = 32;
pub const XASH_TEXTURE4: C2RustUnnamed_0 = 4;
pub const XASH_TEXTURE3: C2RustUnnamed_0 = 3;
pub const XASH_TEXTURE2: C2RustUnnamed_0 = 2;
pub const XASH_TEXTURE1: C2RustUnnamed_0 = 1;
pub const XASH_TEXTURE0: C2RustUnnamed_0 = 0;
pub const GL_KEEP_UNIT: C2RustUnnamed_0 = -1;
pub type ref_defaultsprite_e = libc::c_uint;
pub const REF_CHROME_SPRITE: ref_defaultsprite_e = 1;
pub const REF_DOT_SPRITE: ref_defaultsprite_e = 0;
pub type ref_graphic_apis_e = libc::c_uint;
pub const REF_D3D: ref_graphic_apis_e = 2;
pub const REF_GL: ref_graphic_apis_e = 1;
pub const REF_SOFTWARE: ref_graphic_apis_e = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SAFE_LAST: C2RustUnnamed_1 = 8;
pub const SAFE_DONTCARE: C2RustUnnamed_1 = 7;
pub const SAFE_NOCOLOR: C2RustUnnamed_1 = 6;
pub const SAFE_NODEPTH: C2RustUnnamed_1 = 5;
pub const SAFE_NOALPHA: C2RustUnnamed_1 = 4;
pub const SAFE_NOSTENCIL: C2RustUnnamed_1 = 3;
pub const SAFE_NOACC: C2RustUnnamed_1 = 2;
pub const SAFE_NOMSAA: C2RustUnnamed_1 = 1;
pub const SAFE_NO: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const REF_GL_ATTRIBUTES_COUNT: C2RustUnnamed_2 = 20;
pub const REF_GL_CONTEXT_NO_ERROR: C2RustUnnamed_2 = 19;
pub const REF_GL_CONTEXT_RESET_NOTIFICATION: C2RustUnnamed_2 = 18;
pub const REF_GL_CONTEXT_RELEASE_BEHAVIOR: C2RustUnnamed_2 = 17;
pub const REF_GL_FRAMEBUFFER_SRGB_CAPABLE: C2RustUnnamed_2 = 16;
pub const REF_GL_SHARE_WITH_CURRENT_CONTEXT: C2RustUnnamed_2 = 15;
pub const REF_GL_CONTEXT_PROFILE_MASK: C2RustUnnamed_2 = 14;
pub const REF_GL_CONTEXT_FLAGS: C2RustUnnamed_2 = 13;
pub const REF_GL_CONTEXT_EGL: C2RustUnnamed_2 = 12;
pub const REF_GL_CONTEXT_MINOR_VERSION: C2RustUnnamed_2 = 11;
pub const REF_GL_CONTEXT_MAJOR_VERSION: C2RustUnnamed_2 = 10;
pub const REF_GL_ACCELERATED_VISUAL: C2RustUnnamed_2 = 9;
pub const REF_GL_MULTISAMPLESAMPLES: C2RustUnnamed_2 = 8;
pub const REF_GL_MULTISAMPLEBUFFERS: C2RustUnnamed_2 = 7;
pub const REF_GL_STENCIL_SIZE: C2RustUnnamed_2 = 6;
pub const REF_GL_DEPTH_SIZE: C2RustUnnamed_2 = 5;
pub const REF_GL_DOUBLEBUFFER: C2RustUnnamed_2 = 4;
pub const REF_GL_ALPHA_SIZE: C2RustUnnamed_2 = 3;
pub const REF_GL_BLUE_SIZE: C2RustUnnamed_2 = 2;
pub const REF_GL_GREEN_SIZE: C2RustUnnamed_2 = 1;
pub const REF_GL_RED_SIZE: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const REF_GL_CONTEXT_PROFILE_ES: C2RustUnnamed_3 = 4;
pub const REF_GL_CONTEXT_PROFILE_COMPATIBILITY: C2RustUnnamed_3 = 2;
pub const REF_GL_CONTEXT_PROFILE_CORE: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const REF_GL_CONTEXT_RESET_ISOLATION_FLAG: C2RustUnnamed_4 = 8;
pub const REF_GL_CONTEXT_ROBUST_ACCESS_FLAG: C2RustUnnamed_4 = 4;
pub const REF_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: C2RustUnnamed_4 = 2;
pub const REF_GL_CONTEXT_DEBUG_FLAG: C2RustUnnamed_4 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remap_info_s {
    pub textures: [libc::c_ushort; 32],
    pub ptexture: *mut mstudiotex_s,
    pub numtextures: libc::c_short,
    pub topcolor: libc::c_short,
    pub bottomcolor: libc::c_short,
    pub model: *mut model_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ref_api_s {
    pub EngineGetParm: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub Cvar_Get: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char,
                                              _: libc::c_int,
                                              _: *const libc::c_char)
                             -> *mut cvar_t>,
    pub pfnGetCvarPointer: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: libc::c_int)
                                      -> *mut cvar_t>,
    pub pfnGetCvarFloat: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> libc::c_float>,
    pub pfnGetCvarString: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                     -> *const libc::c_char>,
    pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_float) -> ()>,
    pub Cvar_Set: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *const libc::c_char) -> ()>,
    pub Cvar_RegisterVariable: Option<unsafe extern "C" fn(_: *mut cvar_t)
                                          -> ()>,
    pub Cvar_FullSet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int) -> ()>,
    pub Cmd_AddCommand: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _:
                                                        Option<unsafe extern "C" fn()
                                                                   -> ()>,
                                                    _: *const libc::c_char)
                                   -> libc::c_int>,
    pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> ()>,
    pub Cmd_Argc: Option<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option<unsafe extern "C" fn(_: libc::c_int)
                             -> *const libc::c_char>,
    pub Cmd_Args: Option<unsafe extern "C" fn() -> *const libc::c_char>,
    pub Cbuf_AddText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                 -> ()>,
    pub Cbuf_InsertText: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                    -> ()>,
    pub Cbuf_Execute: Option<unsafe extern "C" fn() -> ()>,
    pub Con_Printf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub Con_DPrintf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_Reportf: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NPrintf: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *const libc::c_char,
                                                 _: ...) -> ()>,
    pub Con_NXPrintf: Option<unsafe extern "C" fn(_: *mut con_nprint_s,
                                                  _: *const libc::c_char,
                                                  _: ...) -> ()>,
    pub CL_CenterPrint: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: libc::c_float) -> ()>,
    pub Con_DrawStringLen: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut libc::c_int,
                                                       _: *mut libc::c_int)
                                      -> ()>,
    pub Con_DrawString: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *const libc::c_char,
                                                    _: *mut byte)
                                   -> libc::c_int>,
    pub CL_DrawCenterPrint: Option<unsafe extern "C" fn() -> ()>,
    pub GetLocalPlayer: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetViewModel: Option<unsafe extern "C" fn() -> *mut cl_entity_s>,
    pub GetEntityByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                     -> *mut cl_entity_s>,
    pub R_BeamGetEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut cl_entity_s>,
    pub CL_GetWaterEntity: Option<unsafe extern "C" fn(_: *const vec_t)
                                      -> *mut cl_entity_s>,
    pub CL_AddVisibleEntity: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                         _: libc::c_int)
                                        -> qboolean>,
    pub Mod_SampleSizeForFace: Option<unsafe extern "C" fn(_: *mut msurface_s)
                                          -> libc::c_int>,
    pub Mod_BoxVisible: Option<unsafe extern "C" fn(_: *const vec_t,
                                                    _: *const vec_t,
                                                    _: *const byte)
                                   -> qboolean>,
    pub GetWorld: Option<unsafe extern "C" fn() -> *mut world_static_s>,
    pub Mod_PointInLeaf: Option<unsafe extern "C" fn(_: *const vec_t,
                                                     _: *mut mnode_t)
                                    -> *mut mleaf_t>,
    pub Mod_CreatePolygonsForHull: Option<unsafe extern "C" fn(_: libc::c_int)
                                              -> ()>,
    pub R_StudioSlerpBones: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: *mut vec4_t,
                                                        _:
                                                            *mut [libc::c_float; 3],
                                                        _: libc::c_float)
                                       -> ()>,
    pub R_StudioCalcBoneQuaternion: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_float,
                                                                _:
                                                                    *mut mstudiobone_t,
                                                                _:
                                                                    *mut mstudioanim_t,
                                                                _:
                                                                    *mut libc::c_float,
                                                                _: *mut vec_t)
                                               -> ()>,
    pub R_StudioCalcBonePosition: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  libc::c_float,
                                                              _:
                                                                  *mut mstudiobone_t,
                                                              _:
                                                                  *mut mstudioanim_t,
                                                              _: *mut vec_t,
                                                              _: *mut vec_t)
                                             -> ()>,
    pub R_StudioGetAnim: Option<unsafe extern "C" fn(_: *mut studiohdr_t,
                                                     _: *mut model_t,
                                                     _: *mut mstudioseqdesc_t)
                                    -> *mut libc::c_void>,
    pub pfnStudioEvent: Option<unsafe extern "C" fn(_: *const mstudioevent_s,
                                                    _: *const cl_entity_t)
                                   -> ()>,
    pub CL_DrawEFX: Option<unsafe extern "C" fn(_: libc::c_float, _: qboolean)
                               -> ()>,
    pub CL_ThinkParticle: Option<unsafe extern "C" fn(_: libc::c_double,
                                                      _: *mut particle_t)
                                     -> ()>,
    pub R_FreeDeadParticles: Option<unsafe extern "C" fn(_:
                                                             *mut *mut particle_t)
                                        -> ()>,
    pub CL_AllocParticleFast: Option<unsafe extern "C" fn()
                                         -> *mut particle_t>,
    pub CL_AllocElight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_s>,
    pub GetDefaultSprite: Option<unsafe extern "C" fn(_: ref_defaultsprite_e)
                                     -> *mut model_s>,
    pub R_StoreEfrags: Option<unsafe extern "C" fn(_: *mut *mut efrag_s,
                                                   _: libc::c_int) -> ()>,
    pub Mod_ForName: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                 _: qboolean, _: qboolean)
                                -> *mut model_t>,
    pub Mod_Extradata: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: *mut model_t)
                                  -> *mut libc::c_void>,
    pub pfnGetModelByIndex: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut model_s>,
    pub Mod_GetCurrentLoadingModel: Option<unsafe extern "C" fn()
                                               -> *mut model_s>,
    pub Mod_SetCurrentLoadingModel: Option<unsafe extern "C" fn(_:
                                                                    *mut model_s)
                                               -> ()>,
    pub CL_GetRemapInfoForEntity: Option<unsafe extern "C" fn(_:
                                                                  *mut cl_entity_t)
                                             -> *mut remap_info_s>,
    pub CL_AllocRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                       _: *mut model_t,
                                                       _: libc::c_int,
                                                       _: libc::c_int) -> ()>,
    pub CL_FreeRemapInfo: Option<unsafe extern "C" fn(_: *mut remap_info_s)
                                     -> ()>,
    pub CL_UpdateRemapInfo: Option<unsafe extern "C" fn(_: *mut cl_entity_t,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
    pub CL_ExtraUpdate: Option<unsafe extern "C" fn() -> ()>,
    pub Host_Error: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                _: ...) -> ()>,
    pub COM_SetRandomSeed: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    pub COM_RandomFloat: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float)
                                    -> libc::c_float>,
    pub COM_RandomLong: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int)
                                   -> libc::c_int>,
    pub GetScreenFade: Option<unsafe extern "C" fn() -> *mut screenfade_s>,
    pub pfnTextMessageGet: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut client_textmessage_s>,
    pub GetPredictedOrigin: Option<unsafe extern "C" fn(_: *mut vec_t) -> ()>,
    pub CL_GetPaletteColor: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> *mut color24>,
    pub CL_GetScreenInfo: Option<unsafe extern "C" fn(_: *mut libc::c_int,
                                                      _: *mut libc::c_int)
                                     -> ()>,
    pub SetLocalLightLevel: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
    pub Sys_CheckParm: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                  -> libc::c_int>,
    pub pfnPlayerInfo: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut player_info_t>,
    pub pfnGetPlayerState: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> *mut entity_state_t>,
    pub Mod_CacheCheck: Option<unsafe extern "C" fn(_: *mut cache_user_s)
                                   -> *mut libc::c_void>,
    pub Mod_LoadCacheFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: *mut cache_user_s)
                                      -> ()>,
    pub Mod_Calloc: Option<unsafe extern "C" fn(_: libc::c_int, _: size_t)
                               -> *mut libc::c_void>,
    pub pfnGetStudioModelInterface: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut *mut r_studio_interface_s,
                                                                _:
                                                                    *mut engine_studio_api_s)
                                               -> libc::c_int>,
    pub _Mem_AllocPool: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                    _: *const libc::c_char,
                                                    _: libc::c_int)
                                   -> poolhandle_t>,
    pub _Mem_FreePool: Option<unsafe extern "C" fn(_: *mut poolhandle_t,
                                                   _: *const libc::c_char,
                                                   _: libc::c_int) -> ()>,
    pub _Mem_Alloc: Option<unsafe extern "C" fn(_: poolhandle_t, _: size_t,
                                                _: qboolean,
                                                _: *const libc::c_char,
                                                _: libc::c_int)
                               -> *mut libc::c_void>,
    pub _Mem_Realloc: Option<unsafe extern "C" fn(_: poolhandle_t,
                                                  _: *mut libc::c_void,
                                                  _: size_t, _: qboolean,
                                                  _: *const libc::c_char,
                                                  _: libc::c_int)
                                 -> *mut libc::c_void>,
    pub _Mem_Free: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *const libc::c_char,
                                               _: libc::c_int) -> ()>,
    pub COM_LoadLibrary: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                     _: libc::c_int,
                                                     _: qboolean)
                                    -> *mut libc::c_void>,
    pub COM_FreeLibrary: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()>,
    pub COM_GetProcAddress: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                        _:
                                                            *const libc::c_char)
                                       -> *mut libc::c_void>,
    pub COM_LoadFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut fs_offset_t,
                                                  _: qboolean) -> *mut byte>,
    pub FS_FileExists: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: libc::c_int)
                                  -> libc::c_int>,
    pub FS_AllowDirectPaths: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub R_Init_Video: Option<unsafe extern "C" fn(_: libc::c_int)
                                 -> qboolean>,
    pub R_Free_Video: Option<unsafe extern "C" fn() -> ()>,
    pub GL_SetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetAttribute: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_int)
                                    -> libc::c_int>,
    pub GL_GetProcAddress: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> *mut libc::c_void>,
    pub GL_SwapBuffers: Option<unsafe extern "C" fn() -> ()>,
    pub SW_CreateBuffer: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_int,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint,
                                                     _: *mut uint)
                                    -> qboolean>,
    pub SW_LockBuffer: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub SW_UnlockBuffer: Option<unsafe extern "C" fn() -> ()>,
    pub BuildGammaTable: Option<unsafe extern "C" fn(_: libc::c_float,
                                                     _: libc::c_float) -> ()>,
    pub LightToTexGamma: Option<unsafe extern "C" fn(_: byte) -> byte>,
    pub R_DoResetGamma: Option<unsafe extern "C" fn() -> qboolean>,
    pub GetLightStyle: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut lightstyle_t>,
    pub GetDynamicLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *mut dlight_t>,
    pub GetEntityLight: Option<unsafe extern "C" fn(_: libc::c_int)
                                   -> *mut dlight_t>,
    pub R_FatPVS: Option<unsafe extern "C" fn(_: *const libc::c_float,
                                              _: libc::c_float, _: *mut byte,
                                              _: qboolean, _: qboolean)
                             -> libc::c_int>,
    pub GetOverviewParms: Option<unsafe extern "C" fn()
                                     -> *const ref_overview_s>,
    pub pfnTime: Option<unsafe extern "C" fn() -> libc::c_double>,
    pub EV_GetPhysent: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut physent_s>,
    pub EV_TraceSurface: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_float,
                                                     _: *mut libc::c_float)
                                    -> *mut msurface_s>,
    pub PM_TraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                  _: *mut libc::c_float,
                                                  _: libc::c_int,
                                                  _: libc::c_int,
                                                  _: libc::c_int)
                                 -> *mut pmtrace_s>,
    pub EV_VisTraceLine: Option<unsafe extern "C" fn(_: *mut libc::c_float,
                                                     _: *mut libc::c_float,
                                                     _: libc::c_int)
                                    -> *mut pmtrace_s>,
    pub CL_TraceLine: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                  _: *mut vec_t,
                                                  _: libc::c_int)
                                 -> pmtrace_s>,
    pub pfnGetMoveVars: Option<unsafe extern "C" fn() -> *mut movevars_s>,
    pub Image_AddCmdFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_SetForceFlags: Option<unsafe extern "C" fn(_: uint) -> ()>,
    pub Image_ClearForceFlags: Option<unsafe extern "C" fn() -> ()>,
    pub Image_CustomPalette: Option<unsafe extern "C" fn() -> qboolean>,
    pub Image_Process: Option<unsafe extern "C" fn(_: *mut *mut rgbdata_t,
                                                   _: libc::c_int,
                                                   _: libc::c_int, _: uint,
                                                   _: libc::c_float)
                                  -> qboolean>,
    pub FS_LoadImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *const byte, _: size_t)
                                 -> *mut rgbdata_t>,
    pub FS_SaveImage: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                  _: *mut rgbdata_t)
                                 -> qboolean>,
    pub FS_CopyImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t)
                                 -> *mut rgbdata_t>,
    pub FS_FreeImage: Option<unsafe extern "C" fn(_: *mut rgbdata_t) -> ()>,
    pub Image_SetMDLPointer: Option<unsafe extern "C" fn(_: *mut byte) -> ()>,
    pub Image_GetPool: Option<unsafe extern "C" fn() -> poolhandle_t>,
    pub Image_GetPFDesc: Option<unsafe extern "C" fn(_: libc::c_int)
                                    -> *const bpc_desc_s>,
    pub pfnDrawNormalTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub pfnDrawTransparentTriangles: Option<unsafe extern "C" fn() -> ()>,
    pub drawFuncs: *mut render_interface_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct movevars_s {
    pub gravity: libc::c_float,
    pub stopspeed: libc::c_float,
    pub maxspeed: libc::c_float,
    pub spectatormaxspeed: libc::c_float,
    pub accelerate: libc::c_float,
    pub airaccelerate: libc::c_float,
    pub wateraccelerate: libc::c_float,
    pub friction: libc::c_float,
    pub edgefriction: libc::c_float,
    pub waterfriction: libc::c_float,
    pub entgravity: libc::c_float,
    pub bounce: libc::c_float,
    pub stepsize: libc::c_float,
    pub maxvelocity: libc::c_float,
    pub zmax: libc::c_float,
    pub waveHeight: libc::c_float,
    pub footsteps: qboolean,
    pub skyName: [libc::c_char; 32],
    pub rollangle: libc::c_float,
    pub rollspeed: libc::c_float,
    pub skycolor_r: libc::c_float,
    pub skycolor_g: libc::c_float,
    pub skycolor_b: libc::c_float,
    pub skyvec_x: libc::c_float,
    pub skyvec_y: libc::c_float,
    pub skyvec_z: libc::c_float,
    pub features: libc::c_int,
    pub fog_settings: libc::c_int,
    pub wateralpha: libc::c_float,
    pub skydir_x: libc::c_float,
    pub skydir_y: libc::c_float,
    pub skydir_z: libc::c_float,
    pub skyangle: libc::c_float,
}
pub type ref_api_t = ref_api_s;
pub type GLenum = uint;
pub type GLboolean = byte;
pub type GLbitfield = uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLubyte = byte;
pub type GLuint = uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
pub type GLintptrARB = libc::c_int;
pub type GLsizeiptrARB = libc::c_int;
pub type GLcharARB = libc::c_char;
pub type GL_DEBUG_PROC_ARB
    =
    Option<unsafe extern "C" fn(_: GLenum, _: GLenum, _: GLuint, _: GLenum,
                                _: GLsizei, _: *const GLcharARB,
                                _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct draw_list_t {
    pub solid_entities: [*mut cl_entity_t; 2048],
    pub trans_entities: [*mut cl_entity_t; 2048],
    pub beam_entities: [*mut cl_entity_t; 2048],
    pub num_solid_entities: uint,
    pub num_trans_entities: uint,
    pub num_beam_entities: uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_globals_t {
    pub defaultTexture: libc::c_int,
    pub particleTexture: libc::c_int,
    pub whiteTexture: libc::c_int,
    pub grayTexture: libc::c_int,
    pub blackTexture: libc::c_int,
    pub solidskyTexture: libc::c_int,
    pub alphaskyTexture: libc::c_int,
    pub lightmapTextures: [libc::c_int; 256],
    pub dlightTexture: libc::c_int,
    pub skyboxTextures: [libc::c_int; 6],
    pub cinTexture: libc::c_int,
    pub skytexturenum: libc::c_int,
    pub skyboxbasenum: libc::c_int,
    pub draw_stack: [draw_list_t; 2],
    pub draw_stack_pos: libc::c_int,
    pub draw_list: *mut draw_list_t,
    pub draw_decals: [*mut msurface_t; 4096],
    pub num_draw_decals: libc::c_int,
    pub modelviewIdentity: qboolean,
    pub visframecount: libc::c_int,
    pub dlightframecount: libc::c_int,
    pub realframecount: libc::c_int,
    pub framecount: libc::c_int,
    pub ignore_lightgamma: qboolean,
    pub fCustomRendering: qboolean,
    pub fResetVis: qboolean,
    pub fFlipViewModel: qboolean,
    pub visbytes: [byte; 4096],
    pub lightstylevalue: [libc::c_int; 64],
    pub block_size: libc::c_int,
    pub frametime: libc::c_double,
    pub blend: libc::c_float,
    pub modelorg: vec3_t,
    pub fCustomSkybox: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glconfig_t {
    pub renderer_string: *const libc::c_char,
    pub vendor_string: *const libc::c_char,
    pub version_string: *const libc::c_char,
    pub hardware_type: glHWType_t,
    pub extensions_string: *const libc::c_char,
    pub extension: [byte; 22],
    pub max_texture_units: libc::c_int,
    pub max_texture_coords: libc::c_int,
    pub max_teximage_units: libc::c_int,
    pub max_2d_texture_size: GLint,
    pub max_2d_rectangle_size: GLint,
    pub max_2d_texture_layers: GLint,
    pub max_3d_texture_size: GLint,
    pub max_cubemap_size: GLint,
    pub max_texture_anisotropy: GLfloat,
    pub max_texture_lod_bias: GLfloat,
    pub max_vertex_uniforms: GLint,
    pub max_vertex_attribs: GLint,
    pub max_multisamples: GLint,
    pub color_bits: libc::c_int,
    pub alpha_bits: libc::c_int,
    pub depth_bits: libc::c_int,
    pub stencil_bits: libc::c_int,
    pub msaasamples: libc::c_int,
    pub context: gl_context_type_t,
    pub wrapper: gles_wrapper_t,
    pub softwareGammaUpdate: qboolean,
    pub fCustomRenderer: qboolean,
    pub prev_width: libc::c_int,
    pub prev_height: libc::c_int,
}
pub type glHWType_t = libc::c_uint;
pub const GLHW_INTEL: glHWType_t = 3;
pub const GLHW_NVIDIA: glHWType_t = 2;
pub const GLHW_RADEON: glHWType_t = 1;
pub const GLHW_GENERIC: glHWType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glstate_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub activeTMU: libc::c_int,
    pub currentTextures: [GLint; 32],
    pub currentTextureTargets: [GLuint; 32],
    pub texIdentityMatrix: [GLboolean; 32],
    pub genSTEnabled: [GLint; 32],
    pub texCoordArrayMode: [GLint; 32],
    pub isFogEnabled: GLint,
    pub faceCull: libc::c_int,
    pub stencilEnabled: qboolean,
    pub in2DMode: qboolean,
}
pub const GL_SHADER_GLSL100_EXT: C2RustUnnamed_5 = 6;
pub const GL_EXTCOUNT: C2RustUnnamed_5 = 22;
pub const GL_TEXTURE_ARRAY_EXT: C2RustUnnamed_5 = 8;
pub const GL_TEXTURE_2D_RECT_EXT: C2RustUnnamed_5 = 7;
pub const GL_ANISOTROPY_EXT: C2RustUnnamed_5 = 3;
pub const GL_TEXTURE_CUBEMAP_EXT: C2RustUnnamed_5 = 2;
pub const GL_ARB_MULTITEXTURE: C2RustUnnamed_5 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glwstate_t {
    pub initialized: qboolean,
    pub extended: qboolean,
}
pub const GL_TEXTURE_COMPRESSION_EXT: C2RustUnnamed_5 = 5;
pub const GL_DEBUG_OUTPUT: C2RustUnnamed_5 = 18;
pub const GL_OPENGL_110: C2RustUnnamed_5 = 0;
pub const GL_DRAW_RANGEELEMENTS_EXT: C2RustUnnamed_5 = 20;
pub const GL_TEXTURE_MULTISAMPLE: C2RustUnnamed_5 = 21;
pub const GL_ARB_VERTEX_BUFFER_OBJECT_EXT: C2RustUnnamed_5 = 19;
pub const GL_EXT_GPU_SHADER4: C2RustUnnamed_5 = 16;
pub const GL_ARB_DEPTH_FLOAT_EXT: C2RustUnnamed_5 = 14;
pub const GL_ARB_TEXTURE_FLOAT_EXT: C2RustUnnamed_5 = 13;
pub const GL_DEPTH_TEXTURE: C2RustUnnamed_5 = 17;
pub const GL_CLAMP_TEXBORDER_EXT: C2RustUnnamed_5 = 12;
pub const GL_TEXTURE_LOD_BIAS: C2RustUnnamed_5 = 4;
pub const GL_CLAMPTOEDGE_EXT: C2RustUnnamed_5 = 10;
pub const GL_ARB_TEXTURE_NPOT_EXT: C2RustUnnamed_5 = 11;
pub const GL_ARB_SEAMLESS_CUBEMAP: C2RustUnnamed_5 = 15;
pub const GL_TEXTURE_3D_EXT: C2RustUnnamed_5 = 9;
pub type C2RustUnnamed_5 = libc::c_uint;
// XASH_GL4ES
#[no_mangle]
pub static mut gl_extensions: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texture_anisotropy: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texture_lodbias: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texture_nearest: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_lightmap_nearest: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_keeptjunctions: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_emboss_scale: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_check_errors: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_polyoffset: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_wireframe: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_finish: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_nosort: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_vsync: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_clear: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_test: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_msaa: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_stencilbits: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_fullbright: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_norefresh: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lighting_extended: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lighting_modulate: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lighting_ambient: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_detailtextures: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawentities: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_adjust_fov: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_decals: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_novis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_nocull: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lockpvs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lockfrustum: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_traceglow: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_dynamic: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lightmap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_showhull: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_round_down: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_vbo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_vbo_dlightmode: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_showtextures: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_lightstyle_lerping: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_brightness: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_gamma: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracerred: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracergreen: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut tracerblue: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut traceralpha: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_temppool: poolhandle_t = 0;
#[no_mangle]
pub static mut tr: gl_globals_t =
    gl_globals_t{defaultTexture: 0,
                 particleTexture: 0,
                 whiteTexture: 0,
                 grayTexture: 0,
                 blackTexture: 0,
                 solidskyTexture: 0,
                 alphaskyTexture: 0,
                 lightmapTextures: [0; 256],
                 dlightTexture: 0,
                 skyboxTextures: [0; 6],
                 cinTexture: 0,
                 skytexturenum: 0,
                 skyboxbasenum: 0,
                 draw_stack:
                     [draw_list_t{solid_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  trans_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  beam_entities:
                                      [0 as *const cl_entity_t as
                                           *mut cl_entity_t; 2048],
                                  num_solid_entities: 0,
                                  num_trans_entities: 0,
                                  num_beam_entities: 0,}; 2],
                 draw_stack_pos: 0,
                 draw_list: 0 as *const draw_list_t as *mut draw_list_t,
                 draw_decals:
                     [0 as *const msurface_t as *mut msurface_t; 4096],
                 num_draw_decals: 0,
                 modelviewIdentity: false_0,
                 visframecount: 0,
                 dlightframecount: 0,
                 realframecount: 0,
                 framecount: 0,
                 ignore_lightgamma: false_0,
                 fCustomRendering: false_0,
                 fResetVis: false_0,
                 fFlipViewModel: false_0,
                 visbytes: [0; 4096],
                 lightstylevalue: [0; 64],
                 block_size: 0,
                 frametime: 0.,
                 blend: 0.,
                 modelorg: [0.; 3],
                 fCustomSkybox: false_0,};
#[no_mangle]
pub static mut glConfig: glconfig_t =
    glconfig_t{renderer_string: 0 as *const libc::c_char,
               vendor_string: 0 as *const libc::c_char,
               version_string: 0 as *const libc::c_char,
               hardware_type: GLHW_GENERIC,
               extensions_string: 0 as *const libc::c_char,
               extension: [0; 22],
               max_texture_units: 0,
               max_texture_coords: 0,
               max_teximage_units: 0,
               max_2d_texture_size: 0,
               max_2d_rectangle_size: 0,
               max_2d_texture_layers: 0,
               max_3d_texture_size: 0,
               max_cubemap_size: 0,
               max_texture_anisotropy: 0.,
               max_texture_lod_bias: 0.,
               max_vertex_uniforms: 0,
               max_vertex_attribs: 0,
               max_multisamples: 0,
               color_bits: 0,
               alpha_bits: 0,
               depth_bits: 0,
               stencil_bits: 0,
               msaasamples: 0,
               context: CONTEXT_TYPE_GL,
               wrapper: GLES_WRAPPER_NONE,
               softwareGammaUpdate: false_0,
               fCustomRenderer: false_0,
               prev_width: 0,
               prev_height: 0,};
#[no_mangle]
pub static mut glState: glstate_t =
    glstate_t{width: 0,
              height: 0,
              activeTMU: 0,
              currentTextures: [0; 32],
              currentTextureTargets: [0; 32],
              texIdentityMatrix: [0; 32],
              genSTEnabled: [0; 32],
              texCoordArrayMode: [0; 32],
              isFogEnabled: 0,
              faceCull: 0,
              stencilEnabled: false_0,
              in2DMode: false_0,};
#[no_mangle]
pub static mut glw_state: glwstate_t =
    glwstate_t{initialized: false_0, extended: false_0,};
static mut opengl_110funcs: [dllfunc_t; 125] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glClearColor\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClearColor as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLclampf,
                                                                      _:
                                                                          GLclampf,
                                                                      _:
                                                                          GLclampf,
                                                                      _:
                                                                          GLclampf)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLclampf,
                                                                    _:
                                                                        GLclampf,
                                                                    _:
                                                                        GLclampf,
                                                                    _:
                                                                        GLclampf)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClear\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClear as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLbitfield)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLbitfield)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glAlphaFunc\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglAlphaFunc as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLclampf)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLclampf)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glBlendFunc\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBlendFunc as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCullFace\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCullFace as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDrawBuffer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawBuffer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glReadBuffer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglReadBuffer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glAccum\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglAccum as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glEnable\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglEnable as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDisable\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDisable as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glEnableClientState\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglEnableClientState as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDisableClientState\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDisableClientState as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetBooleanv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetBooleanv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLboolean)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *mut GLboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetDoublev\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetDoublev as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *mut GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetFloatv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetFloatv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *mut GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetIntegerv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetIntegerv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *mut GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetError\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetError as
                                   *const Option<unsafe extern "C" fn()
                                                     -> GLenum> as
                                   *mut Option<unsafe extern "C" fn()
                                                   -> GLenum> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetString\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetString as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> *const GLubyte> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> *const GLubyte> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFinish\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFinish as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFlush\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFlush as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClearDepth\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClearDepth as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLclampd)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLclampd)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDepthFunc\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDepthFunc as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDepthMask\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDepthMask as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLboolean)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDepthRange\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDepthRange as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLclampd,
                                                                      _:
                                                                          GLclampd)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLclampd,
                                                                    _:
                                                                        GLclampd)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFrontFace\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFrontFace as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDrawElements\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawElements as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDrawArrays\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawArrays as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColorMask\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColorMask as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLboolean,
                                                                      _:
                                                                          GLboolean,
                                                                      _:
                                                                          GLboolean,
                                                                      _:
                                                                          GLboolean)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLboolean,
                                                                    _:
                                                                        GLboolean,
                                                                    _:
                                                                        GLboolean,
                                                                    _:
                                                                        GLboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glIndexPointer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglIndexPointer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glVertexPointer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglVertexPointer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glNormalPointer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglNormalPointer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColorPointer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColorPointer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoordPointer\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoordPointer as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glArrayElement\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglArrayElement as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor3f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor3f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor3fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor3fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor4f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor4f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor4fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor4fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor3ub\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor3ub as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLubyte,
                                                                      _:
                                                                          GLubyte,
                                                                      _:
                                                                          GLubyte)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLubyte,
                                                                    _:
                                                                        GLubyte,
                                                                    _:
                                                                        GLubyte)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor4ub\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor4ub as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLubyte,
                                                                      _:
                                                                          GLubyte,
                                                                      _:
                                                                          GLubyte,
                                                                      _:
                                                                          GLubyte)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLubyte,
                                                                    _:
                                                                        GLubyte,
                                                                    _:
                                                                        GLubyte,
                                                                    _:
                                                                        GLubyte)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glColor4ubv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglColor4ubv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLubyte)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLubyte)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord1f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord1f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord2f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord2f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord3f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord3f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord4f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord4f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord1fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord1fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord2fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord2fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord3fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord3fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexCoord4fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexCoord4fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexGenf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexGenf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexGenfv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexGenfv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexGeni\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexGeni as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glVertex2f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglVertex2f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glVertex3f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglVertex3f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glVertex3fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglVertex3fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glNormal3f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglNormal3f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glNormal3fv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglNormal3fv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glBegin\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBegin as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glEnd\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglEnd as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glLineWidth\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglLineWidth as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPointSize\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPointSize as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMatrixMode\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMatrixMode as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glOrtho\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglOrtho as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glRasterPos2f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglRasterPos2f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFrustum\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFrustum as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glViewport\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglViewport as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPushMatrix\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPushMatrix as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPopMatrix\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPopMatrix as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPushAttrib\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPushAttrib as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLbitfield)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLbitfield)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPopAttrib\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPopAttrib as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glLoadIdentity\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglLoadIdentity as
                                   *const Option<unsafe extern "C" fn() -> ()>
                                   as
                                   *mut Option<unsafe extern "C" fn() -> ()>
                                   as *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glLoadMatrixd\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglLoadMatrixd as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glLoadMatrixf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglLoadMatrixf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMultMatrixd\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultMatrixd as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMultMatrixf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultMatrixf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glRotated\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglRotated as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glRotatef\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglRotatef as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glScaled\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglScaled as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glScalef\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglScalef as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTranslated\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTranslated as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble,
                                                                      _:
                                                                          GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble,
                                                                    _:
                                                                        GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTranslatef\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTranslatef as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glReadPixels\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglReadPixels as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDrawPixels\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawPixels as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glStencilFunc\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglStencilFunc as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glStencilMask\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglStencilMask as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glStencilOp\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglStencilOp as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClearStencil\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClearStencil as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glIsEnabled\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglIsEnabled as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> GLboolean> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> GLboolean> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glIsList\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglIsList as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLuint)
                                                     -> GLboolean> as
                                   *mut Option<unsafe extern "C" fn(_: GLuint)
                                                   -> GLboolean> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glIsTexture\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglIsTexture as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLuint)
                                                     -> GLboolean> as
                                   *mut Option<unsafe extern "C" fn(_: GLuint)
                                                   -> GLboolean> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexEnvf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexEnvf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexEnvfv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexEnvfv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexEnvi\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexEnvi as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexParameterf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexParameterf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexParameterfv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexParameterfv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexParameteri\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexParameteri as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glHint\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglHint as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPixelStoref\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPixelStoref as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPixelStorei\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPixelStorei as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGenTextures\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGenTextures as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLsizei,
                                                                      _:
                                                                          *mut GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLsizei,
                                                                    _:
                                                                        *mut GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDeleteTextures\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDeleteTextures as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLsizei,
                                                                      _:
                                                                          *const GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLsizei,
                                                                    _:
                                                                        *const GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glBindTexture\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBindTexture as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexImage1D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexImage1D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexImage2D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexImage2D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexSubImage1D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexSubImage1D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexSubImage2D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexSubImage2D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCopyTexImage1D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCopyTexImage1D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCopyTexImage2D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCopyTexImage2D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCopyTexSubImage1D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCopyTexSubImage1D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCopyTexSubImage2D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCopyTexSubImage2D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glScissor\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglScissor as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetTexImage\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetTexImage as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetTexEnviv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetTexEnviv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *mut GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPolygonOffset\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPolygonOffset as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPolygonMode\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPolygonMode as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glPolygonStipple\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglPolygonStipple as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          *const GLubyte)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        *const GLubyte)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClipPlane\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClipPlane as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *const GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *const GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetClipPlane\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetClipPlane as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLdouble)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *mut GLdouble)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glShadeModel\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglShadeModel as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetTexLevelParameteriv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetTexLevelParameteriv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        *mut GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetTexLevelParameterfv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetTexLevelParameterfv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *mut GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        *mut GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFogfv\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFogfv as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          *const GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        *const GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFogf\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFogf as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glFogi\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglFogi as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut debugoutputfuncs: [dllfunc_t; 5] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glDebugMessageControlARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDebugMessageControlARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const GLuint,
                                                                      _:
                                                                          GLboolean)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const GLuint,
                                                                    _:
                                                                        GLboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDebugMessageInsertARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDebugMessageInsertARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLuint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_char)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum,
                                                                    _: GLuint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_char)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDebugMessageCallbackARB\x00" as *const u8
                                   as *const libc::c_char,
                           func:
                               &pglDebugMessageCallbackARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GL_DEBUG_PROC_ARB,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GL_DEBUG_PROC_ARB,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetDebugMessageLogARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetDebugMessageLogARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLuint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *mut GLenum,
                                                                      _:
                                                                          *mut GLenum,
                                                                      _:
                                                                          *mut GLuint,
                                                                      _:
                                                                          *mut GLuint,
                                                                      _:
                                                                          *mut GLsizei,
                                                                      _:
                                                                          *mut libc::c_char)
                                                     -> GLuint> as
                                   *mut Option<unsafe extern "C" fn(_: GLuint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *mut GLenum,
                                                                    _:
                                                                        *mut GLenum,
                                                                    _:
                                                                        *mut GLuint,
                                                                    _:
                                                                        *mut GLuint,
                                                                    _:
                                                                        *mut GLsizei,
                                                                    _:
                                                                        *mut libc::c_char)
                                                   -> GLuint> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut multitexturefuncs: [dllfunc_t; 9] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glMultiTexCoord1f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultiTexCoord1f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMultiTexCoord2f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultiTexCoord2f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMultiTexCoord3f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultiTexCoord3f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMultiTexCoord4f\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMultiTexCoord4f as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat,
                                                                      _:
                                                                          GLfloat)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat,
                                                                    _:
                                                                        GLfloat)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glActiveTexture\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglActiveTexture as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glActiveTextureARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglActiveTextureARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClientActiveTexture\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClientActiveTexture as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glClientActiveTextureARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglClientActiveTextureARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut texture3dextfuncs: [dllfunc_t; 4] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glTexImage3D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexImage3D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glTexSubImage3D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexSubImage3D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCopyTexSubImage3D\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglCopyTexSubImage3D as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut texturecompressionfuncs: [dllfunc_t; 8] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexImage3DARB\x00" as *const u8
                                   as *const libc::c_char,
                           func:
                               &pglCompressedTexImage3DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexImage2DARB\x00" as *const u8
                                   as *const libc::c_char,
                           func:
                               &pglCompressedTexImage2DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexImage1DARB\x00" as *const u8
                                   as *const libc::c_char,
                           func:
                               &pglCompressedTexImage1DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexSubImage3DARB\x00" as
                                   *const u8 as *const libc::c_char,
                           func:
                               &pglCompressedTexSubImage3DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexSubImage2DARB\x00" as
                                   *const u8 as *const libc::c_char,
                           func:
                               &pglCompressedTexSubImage2DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glCompressedTexSubImage1DARB\x00" as
                                   *const u8 as *const libc::c_char,
                           func:
                               &pglCompressedTexSubImage1DARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _: GLint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGetCompressedTexImage\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGetCompressedTexImage as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLint,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLint,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut vbofuncs: [dllfunc_t; 9] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glBindBufferARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBindBufferARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glDeleteBuffersARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDeleteBuffersARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLsizei,
                                                                      _:
                                                                          *const GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLsizei,
                                                                    _:
                                                                        *const GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glGenBuffersARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglGenBuffersARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLsizei,
                                                                      _:
                                                                          *mut GLuint)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        GLsizei,
                                                                    _:
                                                                        *mut GLuint)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glIsBufferARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglIsBufferARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLuint)
                                                     -> GLboolean> as
                                   *mut Option<unsafe extern "C" fn(_: GLuint)
                                                   -> GLboolean> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glMapBufferARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglMapBufferARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLenum)
                                                     -> *mut libc::c_void> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLenum)
                                                   -> *mut libc::c_void> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glUnmapBufferARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglUnmapBufferARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum)
                                                     -> GLboolean> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum)
                                                   -> GLboolean> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glBufferDataARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBufferDataARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizeiptrARB,
                                                                      _:
                                                                          *const libc::c_void,
                                                                      _:
                                                                          GLenum)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLsizeiptrARB,
                                                                    _:
                                                                        *const libc::c_void,
                                                                    _: GLenum)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name:
                               b"glBufferSubDataARB\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglBufferSubDataARB as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLintptrARB,
                                                                      _:
                                                                          GLsizeiptrARB,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLintptrARB,
                                                                    _:
                                                                        GLsizeiptrARB,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut multisampletexfuncs: [dllfunc_t; 2] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glTexImage2DMultisample\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglTexImage2DMultisample as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLboolean)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLsizei,
                                                                    _:
                                                                        GLboolean)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut drawrangeelementsfuncs: [dllfunc_t; 2] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glDrawRangeElements\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawRangeElements as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLuint,
                                                                      _:
                                                                          GLuint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLuint,
                                                                    _: GLuint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
static mut drawrangeelementsextfuncs: [dllfunc_t; 2] =
    unsafe {
        [{
             let mut init =
                 dllfunc_s{name:
                               b"glDrawRangeElementsEXT\x00" as *const u8 as
                                   *const libc::c_char,
                           func:
                               &pglDrawRangeElementsEXT as
                                   *const Option<unsafe extern "C" fn(_:
                                                                          GLenum,
                                                                      _:
                                                                          GLuint,
                                                                      _:
                                                                          GLuint,
                                                                      _:
                                                                          GLsizei,
                                                                      _:
                                                                          GLenum,
                                                                      _:
                                                                          *const libc::c_void)
                                                     -> ()> as
                                   *mut Option<unsafe extern "C" fn(_: GLenum,
                                                                    _: GLuint,
                                                                    _: GLuint,
                                                                    _:
                                                                        GLsizei,
                                                                    _: GLenum,
                                                                    _:
                                                                        *const libc::c_void)
                                                   -> ()> as
                                   *mut *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 dllfunc_s{name: 0 as *const libc::c_char,
                           func:
                               0 as *const *mut libc::c_void as
                                   *mut *mut libc::c_void,};
             init
         }]
    };
/*
========================
DebugCallback

For ARB_debug_output
========================
*/
unsafe extern "C" fn GL_DebugOutput(mut source: GLuint, mut type_0: GLuint,
                                    mut id: GLuint, mut severity: GLuint,
                                    mut length: GLint,
                                    mut message: *const GLcharARB,
                                    mut userParam: *mut libc::c_void) {
    match type_0 {
        33356 => {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3OpenGL Error:^7 %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     message);
        }
        33357 => {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3OpenGL Warning:^7 %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     message);
        }
        33358 => {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3OpenGL Warning:^7 %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     message);
        }
        33359 => {
            gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^3OpenGL Warning:^7 %s\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      message);
        }
        33360 => {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^2OpenGL Note:^7 %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     message);
        }
        33361 | _ => {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^2OpenGL Note:^7 %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     message);
        }
    };
}
/*
=================
GL_SetExtension
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_SetExtension(mut r_ext: libc::c_int,
                                         mut enable: libc::c_int) {
    if r_ext >= 0 as libc::c_int && r_ext < GL_EXTCOUNT as libc::c_int {
        glConfig.extension[r_ext as usize] =
            if enable != 0 { 0x1 as libc::c_int } else { 0 as libc::c_int } as
                byte
    } else {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_SetExtension: invalid extension %d\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 r_ext);
    };
}
/*
=================
GL_Support
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_Support(mut r_ext: libc::c_int) -> qboolean {
    if r_ext >= 0 as libc::c_int && r_ext < GL_EXTCOUNT as libc::c_int {
        return if glConfig.extension[r_ext as usize] as libc::c_int != 0 {
                   true_0 as libc::c_int
               } else { false_0 as libc::c_int } as qboolean
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_Support: invalid extension %d\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             r_ext);
    return false_0;
}
/*
=================
GL_MaxTextureUnits
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_MaxTextureUnits() -> libc::c_int {
    if GL_Support(GL_SHADER_GLSL100_EXT as libc::c_int) as u64 != 0 {
        return if (if glConfig.max_texture_coords >
                          glConfig.max_teximage_units {
                       glConfig.max_texture_coords
                   } else { glConfig.max_teximage_units }) <
                      MAX_TEXTURE_UNITS as libc::c_int {
                   if glConfig.max_texture_coords >
                          glConfig.max_teximage_units {
                       glConfig.max_texture_coords
                   } else { glConfig.max_teximage_units }
               } else { MAX_TEXTURE_UNITS as libc::c_int }
    }
    return glConfig.max_texture_units;
}
/*
=================
GL_CheckExtension
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CheckExtension(mut name: *const libc::c_char,
                                           mut funcs: *const dllfunc_t,
                                           mut cvarname: *const libc::c_char,
                                           mut r_ext: libc::c_int)
 -> qboolean {
    let mut func: *const dllfunc_t = 0 as *const dllfunc_t;
    let mut parm: *mut cvar_t = 0 as *mut cvar_t;
    let mut extensions_string: *const libc::c_char = 0 as *const libc::c_char;
    gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"GL_CheckExtension: %s \x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              name);
    GL_SetExtension(r_ext, true_0 as libc::c_int);
    if !cvarname.is_null() {
        // system config disable extensions
        parm =
            gEngfuncs.Cvar_Get.expect("non-null function pointer")(cvarname,
                                                                   b"1\x00" as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       12 as
                                                                           libc::c_int,
                                                                   va(b"enable or disable %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      name))
    }
    if !parm.is_null() &&
           (if !parm.is_null() && (*parm).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 ||
           (if !gl_extensions.is_null() && (*gl_extensions).value != 0.0f32 {
                true_0 as libc::c_int
            } else { false_0 as libc::c_int }) == 0 &&
               r_ext != GL_OPENGL_110 as libc::c_int {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"- disabled\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
        GL_SetExtension(r_ext, false_0 as libc::c_int);
        return false_0
        // nothing to process at
    } // update render info
    extensions_string = glConfig.extensions_string;
    if (*name.offset(2 as libc::c_int as isize) as libc::c_int == '_' as i32
            ||
            *name.offset(3 as libc::c_int as isize) as libc::c_int ==
                '_' as i32) && Q_strstr(extensions_string, name).is_null() {
        GL_SetExtension(r_ext, false_0 as libc::c_int);
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"- ^1failed\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
        return false_0
    }
    // clear exports
    func = funcs;
    while !func.is_null() && !(*func).name.is_null() {
        *(*func).func = 0 as *mut libc::c_void;
        func = func.offset(1)
    }
    func = funcs;
    while !func.is_null() && !(*func).name.is_null() {
        // functions are cleared before all the extensions are evaluated
        *(*func).func =
            gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")((*func).name);
        if (*(*func).func).is_null() {
            GL_SetExtension(r_ext, false_0 as libc::c_int);
        }
        func = func.offset(1)
        // one or more functions are invalid, extension will be disabled
    }
    if GL_Support(r_ext) as u64 != 0 {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"- ^2enabled\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
        return true_0
    }
    gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"- ^1failed\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
    return false_0;
}
/*
==============
GL_GetProcAddress

defined just for nanogl/glwes, so it don't link to SDL2 directly, nor use dlsym
==============
*/
#[no_mangle]
pub unsafe extern "C" fn GL_GetProcAddress(mut name: *const libc::c_char)
 -> *mut libc::c_void {
    return gEngfuncs.GL_GetProcAddress.expect("non-null function pointer")(name);
}
/*
===============
GL_SetDefaultTexState
===============
*/
unsafe extern "C" fn GL_SetDefaultTexState() {
    let mut i: libc::c_int = 0;
    memset(glState.currentTextures.as_mut_ptr() as *mut libc::c_void,
           -(1 as libc::c_int),
           (MAX_TEXTURE_UNITS as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<GLint>() as
                                                libc::c_ulong));
    memset(glState.texCoordArrayMode.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (MAX_TEXTURE_UNITS as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<GLint>() as
                                                libc::c_ulong));
    memset(glState.genSTEnabled.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (MAX_TEXTURE_UNITS as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<GLint>() as
                                                libc::c_ulong));
    i = 0 as libc::c_int;
    while i < MAX_TEXTURE_UNITS as libc::c_int {
        glState.currentTextureTargets[i as usize] =
            0 as libc::c_int as GLuint;
        glState.texIdentityMatrix[i as usize] =
            true_0 as libc::c_int as GLboolean;
        i += 1
    };
}
/*
===============
GL_SetDefaultState
===============
*/
unsafe extern "C" fn GL_SetDefaultState() {
    memset(&mut glState as *mut glstate_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<glstate_t>() as libc::c_ulong);
    GL_SetDefaultTexState();
    // init draw stack
    tr.draw_list =
        &mut *tr.draw_stack.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut draw_list_t;
    tr.draw_stack_pos = 0 as libc::c_int;
}
/*
===============
GL_SetDefaults
===============
*/
unsafe extern "C" fn GL_SetDefaults() {
    pglFinish.expect("non-null function pointer")();
    pglClearColor.expect("non-null function pointer")(0.5f32, 0.5f32, 0.5f32,
                                                      1.0f32);
    pglDisable.expect("non-null function pointer")(0xb71 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xb44 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xc11 as libc::c_int as
                                                       GLenum);
    pglDepthFunc.expect("non-null function pointer")(0x203 as libc::c_int as
                                                         GLenum);
    pglColor4f.expect("non-null function pointer")(1.0f32, 1.0f32, 1.0f32,
                                                   1.0f32);
    if glState.stencilEnabled as u64 != 0 {
        pglDisable.expect("non-null function pointer")(0xb90 as libc::c_int as
                                                           GLenum);
        pglStencilMask.expect("non-null function pointer")(!(0 as libc::c_int)
                                                               as GLuint);
        pglStencilFunc.expect("non-null function pointer")(0x202 as
                                                               libc::c_int as
                                                               GLenum,
                                                           0 as libc::c_int,
                                                           !(0 as libc::c_int)
                                                               as GLuint);
        pglStencilOp.expect("non-null function pointer")(0x1e00 as libc::c_int
                                                             as GLenum,
                                                         0x1e02 as libc::c_int
                                                             as GLenum,
                                                         0x1e02 as libc::c_int
                                                             as GLenum);
    }
    pglPolygonMode.expect("non-null function pointer")(0x408 as libc::c_int as
                                                           GLenum,
                                                       0x1b02 as libc::c_int
                                                           as GLenum);
    pglPolygonOffset.expect("non-null function pointer")(-1.0f32, -2.0f32);
    GL_CleanupAllTextureUnits();
    pglDisable.expect("non-null function pointer")(0xbe2 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0xbc0 as libc::c_int as
                                                       GLenum);
    pglDisable.expect("non-null function pointer")(0x8037 as libc::c_int as
                                                       GLenum);
    pglAlphaFunc.expect("non-null function pointer")(0x204 as libc::c_int as
                                                         GLenum, 0.0f32);
    pglEnable.expect("non-null function pointer")(0xde1 as libc::c_int as
                                                      GLenum);
    pglShadeModel.expect("non-null function pointer")(0x1d01 as libc::c_int as
                                                          GLenum);
    pglFrontFace.expect("non-null function pointer")(0x901 as libc::c_int as
                                                         GLenum);
    pglPointSize.expect("non-null function pointer")(1.2f32);
    pglLineWidth.expect("non-null function pointer")(1.2f32);
    GL_Cull(0 as libc::c_int as GLenum);
}
/*
=================
R_RenderInfo_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn R_RenderInfo_f() {
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_VENDOR: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             glConfig.vendor_string);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_RENDERER: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             glConfig.renderer_string);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_VERSION: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             glConfig.version_string);
    // don't spam about extensions
    gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"GL_EXTENSIONS: %s\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              glConfig.extensions_string);
    if glConfig.wrapper as libc::c_uint ==
           GLES_WRAPPER_GL4ES as libc::c_int as libc::c_uint {
        let mut vendor: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f00 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut renderer: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f01 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut version: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f02 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut extensions: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f03 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        if !vendor.is_null() {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL4ES_VENDOR: %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     vendor);
        }
        if !renderer.is_null() {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL4ES_RENDERER: %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     renderer);
        }
        if !version.is_null() {
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL4ES_VERSION: %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     version);
        }
        if !extensions.is_null() {
            gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"GL4ES_EXTENSIONS: %s\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      extensions);
        }
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_TEXTURE_SIZE: %i\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             glConfig.max_2d_texture_size);
    if GL_Support(GL_ARB_MULTITEXTURE as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_TEXTURE_UNITS_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_texture_units);
    }
    if GL_Support(GL_TEXTURE_CUBEMAP_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_cubemap_size);
    }
    if GL_Support(GL_ANISOTROPY_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: %.1f\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_texture_anisotropy
                                                                     as
                                                                     libc::c_double);
    }
    if GL_Support(GL_TEXTURE_2D_RECT_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_RECTANGLE_TEXTURE_SIZE: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_2d_rectangle_size);
    }
    if GL_Support(GL_TEXTURE_ARRAY_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_ARRAY_TEXTURE_LAYERS_EXT: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_2d_texture_layers);
    }
    if GL_Support(GL_SHADER_GLSL100_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_TEXTURE_COORDS_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_texture_coords);
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_TEXTURE_IMAGE_UNITS_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_teximage_units);
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_vertex_uniforms);
        gEngfuncs.Con_Printf.expect("non-null function pointer")(b"GL_MAX_VERTEX_ATTRIBS_ARB: %i\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 glConfig.max_vertex_attribs);
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"MODE: %ix%i\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             (*gpGlobals).width,
                                                             (*gpGlobals).height);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"\n\x00" as
                                                                 *const u8 as
                                                                 *const libc::c_char);
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"VERTICAL SYNC: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             if (*gl_vsync).value
                                                                    != 0. {
                                                                 b"enabled\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"disabled\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             });
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"Color %d bits, Alpha %d bits, Depth %d bits, Stencil %d bits\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             glConfig.color_bits,
                                                             glConfig.alpha_bits,
                                                             glConfig.depth_bits,
                                                             glConfig.stencil_bits);
}
#[no_mangle]
pub unsafe extern "C" fn GL_InitExtensionsBigGL() {
    // intialize wrapper type
    glConfig.context = CONTEXT_TYPE_GL;
    glConfig.wrapper = GLES_WRAPPER_NONE;
    if !Q_stristr(glConfig.renderer_string,
                  b"geforce\x00" as *const u8 as
                      *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_NVIDIA
    } else if !Q_stristr(glConfig.renderer_string,
                         b"quadro fx\x00" as *const u8 as
                             *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_NVIDIA
    } else if !Q_stristr(glConfig.renderer_string,
                         b"rv770\x00" as *const u8 as
                             *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_RADEON
    } else if !Q_stristr(glConfig.renderer_string,
                         b"radeon hd\x00" as *const u8 as
                             *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_RADEON
    } else if !Q_stristr(glConfig.renderer_string,
                         b"eah4850\x00" as *const u8 as
                             *const libc::c_char).is_null() ||
                  !Q_stristr(glConfig.renderer_string,
                             b"eah4870\x00" as *const u8 as
                                 *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_RADEON
    } else if !Q_stristr(glConfig.renderer_string,
                         b"radeon\x00" as *const u8 as
                             *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_RADEON
    } else if !Q_stristr(glConfig.renderer_string,
                         b"intel\x00" as *const u8 as
                             *const libc::c_char).is_null() {
        glConfig.hardware_type = GLHW_INTEL
    } else { glConfig.hardware_type = GLHW_GENERIC }
    // gl4es may be used system-wide
    if !Q_stristr(glConfig.renderer_string,
                  b"gl4es\x00" as *const u8 as *const libc::c_char).is_null()
       {
        let mut vendor: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f00 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut renderer: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f01 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut version: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f02 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        let mut extensions: *const libc::c_char =
            pglGetString.expect("non-null function pointer")((0x1f03 as
                                                                  libc::c_int
                                                                  |
                                                                  0x10000 as
                                                                      libc::c_int)
                                                                 as GLenum) as
                *const libc::c_char;
        glConfig.wrapper = GLES_WRAPPER_GL4ES
    }
    // multitexture
    glConfig.max_teximage_units = 1 as libc::c_int;
    glConfig.max_texture_coords = glConfig.max_teximage_units;
    glConfig.max_texture_units = glConfig.max_texture_coords;
    if GL_CheckExtension(b"GL_ARB_multitexture\x00" as *const u8 as
                             *const libc::c_char,
                         multitexturefuncs.as_mut_ptr(),
                         b"gl_arb_multitexture\x00" as *const u8 as
                             *const libc::c_char,
                         GL_ARB_MULTITEXTURE as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x84e2 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_texture_units);
    }
    if glConfig.max_texture_units == 1 as libc::c_int {
        GL_SetExtension(GL_ARB_MULTITEXTURE as libc::c_int,
                        false_0 as libc::c_int);
    }
    // 3d texture support
    if GL_CheckExtension(b"GL_EXT_texture3D\x00" as *const u8 as
                             *const libc::c_char,
                         texture3dextfuncs.as_mut_ptr(),
                         b"gl_texture_3d\x00" as *const u8 as
                             *const libc::c_char,
                         GL_TEXTURE_3D_EXT as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x8073 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_3d_texture_size);
        if glConfig.max_3d_texture_size < 32 as libc::c_int {
            GL_SetExtension(GL_TEXTURE_3D_EXT as libc::c_int,
                            false_0 as libc::c_int);
            gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^1Error:^7 GL_EXT_texture3D reported bogus GL_MAX_3D_TEXTURE_SIZE, disabled\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char);
        }
    }
    // 2d texture array support
    if GL_CheckExtension(b"GL_EXT_texture_array\x00" as *const u8 as
                             *const libc::c_char,
                         texture3dextfuncs.as_mut_ptr(),
                         b"gl_texture_2d_array\x00" as *const u8 as
                             *const libc::c_char,
                         GL_TEXTURE_ARRAY_EXT as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x88ff as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_2d_texture_layers);
    }
    // cubemaps support
    if GL_CheckExtension(b"GL_ARB_texture_cube_map\x00" as *const u8 as
                             *const libc::c_char, 0 as *const dllfunc_t,
                         b"gl_texture_cubemap\x00" as *const u8 as
                             *const libc::c_char,
                         GL_TEXTURE_CUBEMAP_EXT as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x851c as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_cubemap_size);
        // check for seamless cubemaps too
        GL_CheckExtension(b"GL_ARB_seamless_cube_map\x00" as *const u8 as
                              *const libc::c_char, 0 as *const dllfunc_t,
                          b"gl_texture_cubemap_seamless\x00" as *const u8 as
                              *const libc::c_char,
                          GL_ARB_SEAMLESS_CUBEMAP as libc::c_int);
    }
    GL_CheckExtension(b"GL_ARB_texture_non_power_of_two\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      b"gl_texture_npot\x00" as *const u8 as
                          *const libc::c_char,
                      GL_ARB_TEXTURE_NPOT_EXT as libc::c_int);
    GL_CheckExtension(b"GL_ARB_texture_compression\x00" as *const u8 as
                          *const libc::c_char,
                      texturecompressionfuncs.as_mut_ptr(),
                      b"gl_texture_dxt_compression\x00" as *const u8 as
                          *const libc::c_char,
                      GL_TEXTURE_COMPRESSION_EXT as libc::c_int);
    if GL_CheckExtension(b"GL_EXT_texture_edge_clamp\x00" as *const u8 as
                             *const libc::c_char, 0 as *const dllfunc_t,
                         b"gl_clamp_to_edge\x00" as *const u8 as
                             *const libc::c_char,
                         GL_CLAMPTOEDGE_EXT as libc::c_int) as u64 == 0 {
        GL_CheckExtension(b"GL_SGIS_texture_edge_clamp\x00" as *const u8 as
                              *const libc::c_char, 0 as *const dllfunc_t,
                          b"gl_clamp_to_edge\x00" as *const u8 as
                              *const libc::c_char,
                          GL_CLAMPTOEDGE_EXT as libc::c_int);
    }
    glConfig.max_texture_anisotropy = 0.0f32;
    if GL_CheckExtension(b"GL_EXT_texture_filter_anisotropic\x00" as *const u8
                             as *const libc::c_char, 0 as *const dllfunc_t,
                         b"gl_texture_anisotropic_filter\x00" as *const u8 as
                             *const libc::c_char,
                         GL_ANISOTROPY_EXT as libc::c_int) as u64 != 0 {
        pglGetFloatv.expect("non-null function pointer")(0x84ff as libc::c_int
                                                             as GLenum,
                                                         &mut glConfig.max_texture_anisotropy);
    }
    // Win32 only drivers?
    if GL_CheckExtension(b"GL_EXT_texture_lod_bias\x00" as *const u8 as
                             *const libc::c_char, 0 as *const dllfunc_t,
                         b"gl_texture_mipmap_biasing\x00" as *const u8 as
                             *const libc::c_char,
                         GL_TEXTURE_LOD_BIAS as libc::c_int) as u64 != 0 {
        pglGetFloatv.expect("non-null function pointer")(0x84fd as libc::c_int
                                                             as GLenum,
                                                         &mut glConfig.max_texture_lod_bias); // don't confuse users
    }
    GL_CheckExtension(b"GL_ARB_texture_border_clamp\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      0 as *const libc::c_char,
                      GL_CLAMP_TEXBORDER_EXT as libc::c_int);
    GL_CheckExtension(b"GL_ARB_depth_texture\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      0 as *const libc::c_char,
                      GL_DEPTH_TEXTURE as libc::c_int);
    GL_CheckExtension(b"GL_ARB_texture_float\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      b"gl_texture_float\x00" as *const u8 as
                          *const libc::c_char,
                      GL_ARB_TEXTURE_FLOAT_EXT as libc::c_int);
    GL_CheckExtension(b"GL_ARB_depth_buffer_float\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      b"gl_texture_depth_float\x00" as *const u8 as
                          *const libc::c_char,
                      GL_ARB_DEPTH_FLOAT_EXT as libc::c_int);
    GL_CheckExtension(b"GL_EXT_gpu_shader4\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      0 as *const libc::c_char,
                      GL_EXT_GPU_SHADER4 as libc::c_int);
    GL_CheckExtension(b"GL_ARB_vertex_buffer_object\x00" as *const u8 as
                          *const libc::c_char, vbofuncs.as_mut_ptr(),
                      b"gl_vertex_buffer_object\x00" as *const u8 as
                          *const libc::c_char,
                      GL_ARB_VERTEX_BUFFER_OBJECT_EXT as libc::c_int);
    GL_CheckExtension(b"GL_ARB_texture_multisample\x00" as *const u8 as
                          *const libc::c_char,
                      multisampletexfuncs.as_mut_ptr(),
                      b"gl_texture_multisample\x00" as *const u8 as
                          *const libc::c_char,
                      GL_TEXTURE_MULTISAMPLE as libc::c_int);
    if GL_CheckExtension(b"GL_ARB_shading_language_100\x00" as *const u8 as
                             *const libc::c_char, 0 as *const dllfunc_t,
                         0 as *const libc::c_char,
                         GL_SHADER_GLSL100_EXT as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x8871 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_texture_coords);
        pglGetIntegerv.expect("non-null function pointer")(0x8872 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_teximage_units);
        // Win32 only drivers?
        pglGetIntegerv.expect("non-null function pointer")(0x8b4a as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_vertex_uniforms);
        pglGetIntegerv.expect("non-null function pointer")(0x8869 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_vertex_attribs);
    } else {
        // check for hardware skinning
        // just get from multitexturing
        glConfig.max_teximage_units = glConfig.max_texture_units;
        glConfig.max_texture_coords = glConfig.max_teximage_units
    }
    // rectangle textures support
    GL_CheckExtension(b"GL_ARB_texture_rectangle\x00" as *const u8 as
                          *const libc::c_char, 0 as *const dllfunc_t,
                      b"gl_texture_rectangle\x00" as *const u8 as
                          *const libc::c_char,
                      GL_TEXTURE_2D_RECT_EXT as libc::c_int);
    if GL_CheckExtension(b"glDrawRangeElements\x00" as *const u8 as
                             *const libc::c_char,
                         drawrangeelementsfuncs.as_mut_ptr(),
                         b"gl_drawrangeelements\x00" as *const u8 as
                             *const libc::c_char,
                         GL_DRAW_RANGEELEMENTS_EXT as libc::c_int) as u64 == 0
       {
        if GL_CheckExtension(b"glDrawRangeElementsEXT\x00" as *const u8 as
                                 *const libc::c_char,
                             drawrangeelementsextfuncs.as_mut_ptr(),
                             b"gl_drawrangelements\x00" as *const u8 as
                                 *const libc::c_char,
                             GL_DRAW_RANGEELEMENTS_EXT as libc::c_int) as u64
               != 0 {
            pglDrawRangeElements = pglDrawRangeElementsEXT
        }
    }
    // this won't work without extended context
    if glw_state.extended as u64 != 0 {
        GL_CheckExtension(b"GL_ARB_debug_output\x00" as *const u8 as
                              *const libc::c_char,
                          debugoutputfuncs.as_mut_ptr(),
                          b"gl_debug_output\x00" as *const u8 as
                              *const libc::c_char,
                          GL_DEBUG_OUTPUT as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_InitExtensions() {
    GL_OnContextCreated();
    // initialize gl extensions
    GL_CheckExtension(b"OpenGL 1.1.0\x00" as *const u8 as *const libc::c_char,
                      opengl_110funcs.as_mut_ptr(), 0 as *const libc::c_char,
                      GL_OPENGL_110 as libc::c_int);
    // get our various GL strings
    glConfig.vendor_string =
        pglGetString.expect("non-null function pointer")(0x1f00 as libc::c_int
                                                             as GLenum) as
            *const libc::c_char;
    glConfig.renderer_string =
        pglGetString.expect("non-null function pointer")(0x1f01 as libc::c_int
                                                             as GLenum) as
            *const libc::c_char;
    glConfig.version_string =
        pglGetString.expect("non-null function pointer")(0x1f02 as libc::c_int
                                                             as GLenum) as
            *const libc::c_char;
    glConfig.extensions_string =
        pglGetString.expect("non-null function pointer")(0x1f03 as libc::c_int
                                                             as GLenum) as
            *const libc::c_char;
    gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"^3Video^7: %s\n\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              glConfig.renderer_string);
    GL_InitExtensionsBigGL();
    pglGetIntegerv.expect("non-null function pointer")(0xd33 as libc::c_int as
                                                           GLenum,
                                                       &mut glConfig.max_2d_texture_size);
    if glConfig.max_2d_texture_size <= 0 as libc::c_int {
        glConfig.max_2d_texture_size = 256 as libc::c_int
    }
    // enable gldebug if allowed
    if GL_Support(GL_DEBUG_OUTPUT as libc::c_int) as u64 != 0 {
        if (*gpGlobals).developer as u64 != 0 {
            gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"Installing GL_DebugOutput...\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char);
            pglDebugMessageCallbackARB.expect("non-null function pointer")(Some(GL_DebugOutput
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             GLuint,
                                                                                                         _:
                                                                                                             GLuint,
                                                                                                         _:
                                                                                                             GLuint,
                                                                                                         _:
                                                                                                             GLuint,
                                                                                                         _:
                                                                                                             GLint,
                                                                                                         _:
                                                                                                             *const GLcharARB,
                                                                                                         _:
                                                                                                             *mut libc::c_void)
                                                                                        ->
                                                                                            ()),
                                                                           0
                                                                               as
                                                                               *mut libc::c_void);
            // force everything to happen in the main thread instead of in a separate driver thread
            pglEnable.expect("non-null function pointer")(0x8242 as
                                                              libc::c_int as
                                                              GLenum);
        }
        // enable all the low priority messages
        pglDebugMessageControlARB.expect("non-null function pointer")(0x1100
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          GLenum,
                                                                      0x1100
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          GLenum,
                                                                      0x9148
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          GLenum,
                                                                      0 as
                                                                          libc::c_int,
                                                                      0 as
                                                                          *const GLuint,
                                                                      true_0
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          GLboolean);
    }
    if GL_Support(GL_TEXTURE_2D_RECT_EXT as libc::c_int) as u64 != 0 {
        pglGetIntegerv.expect("non-null function pointer")(0x84f8 as
                                                               libc::c_int as
                                                               GLenum,
                                                           &mut glConfig.max_2d_rectangle_size);
    }
    gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_max_size\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           va(b"%i\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              glConfig.max_2d_texture_size),
                                                           0 as libc::c_int,
                                                           b"opengl texture max dims\x00"
                                                               as *const u8 as
                                                               *const libc::c_char);
    gEngfuncs.Cvar_Set.expect("non-null function pointer")(b"gl_anisotropy\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           va(b"%f\x00" as
                                                                  *const u8 as
                                                                  *const libc::c_char,
                                                              if (*gl_texture_anisotropy).value
                                                                     >=
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_float
                                                                 {
                                                                  if (*gl_texture_anisotropy).value
                                                                         <
                                                                         glConfig.max_texture_anisotropy
                                                                     {
                                                                      (*gl_texture_anisotropy).value
                                                                  } else {
                                                                      glConfig.max_texture_anisotropy
                                                                  }
                                                              } else {
                                                                  0 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_float
                                                              } as
                                                                  libc::c_double));
    if GL_Support(GL_TEXTURE_COMPRESSION_EXT as libc::c_int) as u64 != 0 {
        gEngfuncs.Image_AddCmdFlags.expect("non-null function pointer")(IL_DDS_HARDWARE
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            uint);
    }
    // MCD has buffering issues
    R_RenderInfo_f();
    tr.visframecount = 1 as libc::c_int;
    tr.framecount = tr.visframecount;
    glw_state.initialized = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn GL_ClearExtensions() {
    // now all extensions are disabled
    memset(glConfig.extension.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[byte; 22]>() as libc::c_ulong);
    glw_state.initialized = false_0;
}
//=======================================================================
/*
=================
GL_InitCommands
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_InitCommands() {
    r_speeds =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_speeds\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"shows renderer speeds\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_fullbright =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_fullbright\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"disable lightmaps, get fullbright for entities\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_norefresh =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_norefresh\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"disable 3D rendering (use with caution)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lighting_extended =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lighting_extended\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"allow to get lighting from world and bmodels\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lighting_modulate =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lighting_modulate\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.6\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"lightstyles modulate scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lighting_ambient =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lighting_ambient\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.3\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"map ambient lighting scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_novis =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_novis\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"ignore vis information (perfomance test)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_nocull =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_nocull\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"ignore frustrum culling (perfomance test)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_detailtextures =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_detailtextures\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"enable detail textures support\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lockpvs =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lockpvs\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"lockpvs area at current point (pvs test)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lockfrustum =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lockfrustum\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"lock frustrum area at current point (cull test)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_dynamic =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_dynamic\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"allow dynamic lighting (dlights, lightstyles)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_traceglow =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_traceglow\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"cull flares behind models\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_lightmap =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_lightmap\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   15 as
                                                                       libc::c_int,
                                                               b"lightmap debugging tool\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_drawentities =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_drawentities\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    r_decals =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_decals\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    r_showhull =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_showhull\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    gl_extensions =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_allow_extensions\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"allow gl_extensions\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_texture_nearest =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_texture_nearest\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"disable texture filter\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_lightmap_nearest =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_lightmap_nearest\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"disable lightmap filter\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_check_errors =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_check_errors\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"ignore video engine errors\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_vsync =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"gl_vsync\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    gl_texture_anisotropy =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_anisotropy\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"textures anisotropic filter\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_texture_lodbias =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_texture_lodbias\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"LOD bias for mipmapped textures (perfomance|quality)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_keeptjunctions =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_keeptjunctions\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"removing tjuncs causes blinking pixels\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_emboss_scale =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_emboss_scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       30 as
                                                                           libc::c_int,
                                                               b"fake bumpmapping scale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_showtextures =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"r_showtextures\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    gl_finish =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_finish\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"use glFinish instead of glFlush\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_nosort =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_nosort\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"disable sorting of translucent surfaces\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_clear =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"gl_clear\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    gl_test =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_test\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"engine developer cvar for quick testing new features\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_wireframe =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_wireframe\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       6 as
                                                                           libc::c_int,
                                                               b"show wireframe overlay\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_msaa =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_msaa\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"1\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"enable or disable multisample anti-aliasing\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_stencilbits =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_stencilbits\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"pixelformat stencil bits (0 - auto)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    gl_round_down =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_round_down\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"2\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   12 as
                                                                       libc::c_int,
                                                               b"round texture sizes to nearest POT value\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    // these cvar not used by engine but some mods requires this
    gl_polyoffset =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"gl_polyoffset\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"2.0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"polygon offset for decals\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    // make sure gl_vsync is checked after vid_restart
    (*gl_vsync).flags =
        (*gl_vsync).flags | (1 as libc::c_int) << 13 as libc::c_int;
    vid_gamma =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"gamma\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    vid_brightness =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"brightness\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    tracerred =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracerred\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer red component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    tracergreen =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracergreen\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.8\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer green component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    tracerblue =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"tracerblue\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.4\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer blue component weight ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    traceralpha =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"traceralpha\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0.5\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               0 as
                                                                   libc::c_int,
                                                               b"tracer alpha amount ( 0 - 1.0 )\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    cl_lightstyle_lerping =
        gEngfuncs.pfnGetCvarPointer.expect("non-null function pointer")(b"cl_lightstyle_lerping\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        0 as
                                                                            libc::c_int);
    gEngfuncs.Cmd_AddCommand.expect("non-null function pointer")(b"r_info\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 Some(R_RenderInfo_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  ()),
                                                                 b"display renderer info\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
    gEngfuncs.Cmd_AddCommand.expect("non-null function pointer")(b"timerefresh\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 Some(SCR_TimeRefresh_f
                                                                          as
                                                                          unsafe extern "C" fn()
                                                                              ->
                                                                                  ()),
                                                                 b"turn quickly and print rendering statistcs\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
}
/*
===============
R_CheckVBO

register VBO cvars and get default value
===============
*/
unsafe extern "C" fn R_CheckVBO() {
    let mut def: *const libc::c_char =
        b"1\x00" as *const u8 as *const libc::c_char;
    let mut dlightmode: *const libc::c_char =
        b"1\x00" as *const u8 as *const libc::c_char;
    let mut flags: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
    let mut disable: qboolean = false_0;
    // some bad GLES1 implementations breaks dlights completely
    if glConfig.max_texture_units < 3 as libc::c_int { disable = true_0 }
    if disable as u64 != 0 {
        // do not keep in config unless dev > 3 and enabled
        flags = 0 as libc::c_int;
        def = b"0\x00" as *const u8 as *const libc::c_char
    }
    r_vbo =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_vbo\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               def, flags,
                                                               b"draw world using VBO\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    r_vbo_dlightmode =
        gEngfuncs.Cvar_Get.expect("non-null function pointer")(b"r_vbo_dlightmode\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               dlightmode,
                                                               (1 as
                                                                    libc::c_int)
                                                                   <<
                                                                   0 as
                                                                       libc::c_int,
                                                               b"vbo dlight rendering mode(0-1)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    // check if enabled manually
    if if !r_vbo.is_null() && (*r_vbo).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } != 0 {
        (*r_vbo).flags |= (1 as libc::c_int) << 0 as libc::c_int
    };
}
/*
=================
GL_RemoveCommands
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_RemoveCommands() {
    gEngfuncs.Cmd_RemoveCommand.expect("non-null function pointer")(b"r_info\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
}
/*
===============
R_Init
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Init() -> qboolean {
    if glw_state.initialized as u64 != 0 { return true_0 }
    GL_InitCommands();
    GL_InitRandomTable();
    GL_SetDefaultState();
    // create the window and set up the context
    if gEngfuncs.R_Init_Video.expect("non-null function pointer")(REF_GL as
                                                                      libc::c_int)
           as u64 == 0 {
        // request GL context
        GL_RemoveCommands();
        gEngfuncs.R_Free_Video.expect("non-null function pointer")();
        // Why? Host_Error again???
//		gEngfuncs.Host_Error( "Can't initialize video subsystem\nProbably driver was not installed" );
        return false_0
    }
    r_temppool =
        gEngfuncs._Mem_AllocPool.expect("non-null function pointer")(b"Render Zone\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"../ref_gl/gl_opengl.c\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     975 as
                                                                         libc::c_int);
    GL_SetDefaults();
    R_CheckVBO();
    R_InitImages();
    R_SpriteInit();
    R_StudioInit();
    R_AliasInit();
    R_ClearDecals();
    R_ClearScene();
    return true_0;
}
/*
===============
R_Shutdown
===============
*/
#[no_mangle]
pub unsafe extern "C" fn R_Shutdown() {
    if glw_state.initialized as u64 == 0 { return }
    GL_RemoveCommands();
    R_ShutdownImages();
    gEngfuncs._Mem_FreePool.expect("non-null function pointer")(&mut r_temppool,
                                                                b"../ref_gl/gl_opengl.c\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                1002 as
                                                                    libc::c_int);
    // XASH_GL4ES
    // shut down OS specific OpenGL stuff like contexts, etc.
    gEngfuncs.R_Free_Video.expect("non-null function pointer")();
}
/*
=================
GL_ErrorString
convert errorcode to string
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_ErrorString(mut err: libc::c_int)
 -> *const libc::c_char {
    match err {
        1283 => {
            return b"GL_STACK_OVERFLOW\x00" as *const u8 as
                       *const libc::c_char
        }
        1284 => {
            return b"GL_STACK_UNDERFLOW\x00" as *const u8 as
                       *const libc::c_char
        }
        1280 => {
            return b"GL_INVALID_ENUM\x00" as *const u8 as *const libc::c_char
        }
        1281 => {
            return b"GL_INVALID_VALUE\x00" as *const u8 as *const libc::c_char
        }
        1282 => {
            return b"GL_INVALID_OPERATION\x00" as *const u8 as
                       *const libc::c_char
        }
        1285 => {
            return b"GL_OUT_OF_MEMORY\x00" as *const u8 as *const libc::c_char
        }
        _ => {
            return b"UNKNOWN ERROR\x00" as *const u8 as *const libc::c_char
        }
    };
}
/*
=================
GL_CheckForErrors
obsolete
=================
*/
#[no_mangle]
pub unsafe extern "C" fn GL_CheckForErrors_(mut filename: *const libc::c_char,
                                            fileline: libc::c_int) {
    let mut err: libc::c_int = 0; // REFTODO!!!!!
    if if !gl_check_errors.is_null() && (*gl_check_errors).value != 0.0f32 {
           true_0 as libc::c_int
       } else { false_0 as libc::c_int } == 0 {
        return
    }
    err = pglGetError.expect("non-null function pointer")() as libc::c_int;
    if err == 0 as libc::c_int { return }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"^3OpenGL Error:^7 %s (at %s:%i)\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             GL_ErrorString(err),
                                                             filename,
                                                             fileline);
}
#[no_mangle]
pub unsafe extern "C" fn GL_SetupAttributes(mut safegl: libc::c_int) {
    let mut context_flags: libc::c_int = 0 as libc::c_int;
    let mut samples: libc::c_int = 0 as libc::c_int;
    // GL1.x
    if gEngfuncs.Sys_CheckParm.expect("non-null function pointer")(b"-glcore\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
           != 0 {
        context_flags =
            (context_flags as libc::c_uint |
                 (1 as libc::c_uint) << 0 as libc::c_int) as libc::c_int;
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_PROFILE_MASK
                                                                          as
                                                                          libc::c_int,
                                                                      REF_GL_CONTEXT_PROFILE_CORE
                                                                          as
                                                                          libc::c_int);
    } else {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_PROFILE_MASK
                                                                          as
                                                                          libc::c_int,
                                                                      REF_GL_CONTEXT_PROFILE_COMPATIBILITY
                                                                          as
                                                                          libc::c_int);
    }
    // XASH_GLES
    if gEngfuncs.Sys_CheckParm.expect("non-null function pointer")(b"-gldebug\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char)
           != 0 {
        gEngfuncs.Con_Reportf.expect("non-null function pointer")(b"Creating an extended GL context for debug...\n\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char); // can't retry anymore, can only shutdown engine
        context_flags =
            (context_flags as libc::c_uint |
                 (1 as libc::c_uint) << 1 as libc::c_int) as libc::c_int;
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_CONTEXT_FLAGS
                                                                          as
                                                                          libc::c_int,
                                                                      REF_GL_CONTEXT_DEBUG_FLAG
                                                                          as
                                                                          libc::c_int);
        glw_state.extended = true_0
    }
    if safegl > SAFE_DONTCARE as libc::c_int {
        safegl = -(1 as libc::c_int);
        return
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"Trying safe opengl mode %d\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             safegl);
    if safegl == SAFE_DONTCARE as libc::c_int { return }
    gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_DOUBLEBUFFER
                                                                      as
                                                                      libc::c_int,
                                                                  1 as
                                                                      libc::c_int);
    if safegl < SAFE_NOACC as libc::c_int {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_ACCELERATED_VISUAL
                                                                          as
                                                                          libc::c_int,
                                                                      1 as
                                                                          libc::c_int);
    }
    gEngfuncs.Con_Printf.expect("non-null function pointer")(b"bpp %d\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             (*gpGlobals).desktopBitsPixel);
    if safegl < SAFE_NOSTENCIL as libc::c_int {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_STENCIL_SIZE
                                                                          as
                                                                          libc::c_int,
                                                                      (*gl_stencilbits).value
                                                                          as
                                                                          libc::c_int);
    }
    if safegl < SAFE_NOALPHA as libc::c_int {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_ALPHA_SIZE
                                                                          as
                                                                          libc::c_int,
                                                                      8 as
                                                                          libc::c_int);
    }
    if safegl < SAFE_NODEPTH as libc::c_int {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_DEPTH_SIZE
                                                                          as
                                                                          libc::c_int,
                                                                      24 as
                                                                          libc::c_int);
    } else {
        gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_DEPTH_SIZE
                                                                          as
                                                                          libc::c_int,
                                                                      8 as
                                                                          libc::c_int);
    }
    if safegl < SAFE_NOCOLOR as libc::c_int {
        if (*gpGlobals).desktopBitsPixel >= 24 as libc::c_int {
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_RED_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          8 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_GREEN_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          8 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_BLUE_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          8 as
                                                                              libc::c_int);
        } else if (*gpGlobals).desktopBitsPixel >= 16 as libc::c_int {
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_RED_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          5 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_GREEN_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          6 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_BLUE_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          5 as
                                                                              libc::c_int);
        } else {
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_RED_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          3 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_GREEN_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          3 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_BLUE_SIZE
                                                                              as
                                                                              libc::c_int,
                                                                          2 as
                                                                              libc::c_int);
        }
    }
    if safegl < SAFE_NOMSAA as libc::c_int {
        match gEngfuncs.pfnGetCvarFloat.expect("non-null function pointer")(b"gl_msaa_samples\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char)
                  as libc::c_int {
            2 | 4 | 8 | 16 => {
                samples =
                    gEngfuncs.pfnGetCvarFloat.expect("non-null function pointer")(b"gl_msaa_samples\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char)
                        as libc::c_int
                // don't use, because invalid parameter is passed
            }
            _ => { samples = 0 as libc::c_int }
        }
        if samples != 0 {
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_MULTISAMPLEBUFFERS
                                                                              as
                                                                              libc::c_int,
                                                                          1 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_MULTISAMPLESAMPLES
                                                                              as
                                                                              libc::c_int,
                                                                          samples);
            glConfig.max_multisamples = samples
        } else {
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_MULTISAMPLEBUFFERS
                                                                              as
                                                                              libc::c_int,
                                                                          0 as
                                                                              libc::c_int);
            gEngfuncs.GL_SetAttribute.expect("non-null function pointer")(REF_GL_MULTISAMPLESAMPLES
                                                                              as
                                                                              libc::c_int,
                                                                          0 as
                                                                              libc::c_int);
            glConfig.max_multisamples = 0 as libc::c_int
        }
    } else {
        gEngfuncs.Cvar_Set.expect("non-null function pointer")(b"gl_msaa_samples\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char,
                                                               b"0\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_OnContextCreated() {
    let mut colorBits: [libc::c_int; 3] = [0; 3];
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_RED_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut *colorBits.as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize));
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_GREEN_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut *colorBits.as_mut_ptr().offset(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize));
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_BLUE_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut *colorBits.as_mut_ptr().offset(2
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize));
    glConfig.color_bits =
        colorBits[0 as libc::c_int as usize] +
            colorBits[1 as libc::c_int as usize] +
            colorBits[2 as libc::c_int as usize];
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_ALPHA_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut glConfig.alpha_bits);
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_DEPTH_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut glConfig.depth_bits);
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_STENCIL_SIZE
                                                                      as
                                                                      libc::c_int,
                                                                  &mut glConfig.stencil_bits);
    glState.stencilEnabled =
        if glConfig.stencil_bits != 0 {
            true_0 as libc::c_int
        } else { false_0 as libc::c_int } as qboolean;
    gEngfuncs.GL_GetAttribute.expect("non-null function pointer")(REF_GL_MULTISAMPLESAMPLES
                                                                      as
                                                                      libc::c_int,
                                                                  &mut glConfig.msaasamples);
}
