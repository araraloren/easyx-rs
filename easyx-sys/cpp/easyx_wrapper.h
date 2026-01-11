// easyx_wrapper.h
// C 风格头文件，声明 EasyX 包装层的函数

#ifndef EASYX_WRAPPER_H
#define EASYX_WRAPPER_H

#include <stddef.h>
#include <stdint.h>
#include <windows.h>

// 前向声明ExMessage结构体
typedef struct ExMessage ExMessage;

// 常量定义
#define EASYX_BLACK 0
#define EASYX_BLUE 0xAA0000
#define EASYX_GREEN 0x00AA00
#define EASYX_CYAN 0xAAAA00
#define EASYX_RED 0x0000AA
#define EASYX_MAGENTA 0xAA00AA
#define EASYX_BROWN 0x0055AA
#define EASYX_LIGHTGRAY 0xAAAAAA
#define EASYX_DARKGRAY 0x555555
#define EASYX_LIGHTBLUE 0xFF5555
#define EASYX_LIGHTGREEN 0x55FF55
#define EASYX_LIGHTCYAN 0xFFFF55
#define EASYX_LIGHTRED 0x5555FF
#define EASYX_LIGHTMAGENTA 0xFF55FF
#define EASYX_YELLOW 0x55FFFF
#define EASYX_WHITE 0xFFFFFF

// 光栅操作模式常量
#define R2_BLACK 1
#define R2_NOTMERGEPEN 2
#define R2_MASKNOTPEN 3
#define R2_NOTCOPYPEN 4
#define R2_MASKPENNOT 5
#define R2_NOT 6
#define R2_XORPEN 7
#define R2_NOTMASKPEN 8
#define R2_MASKPEN 9
#define R2_NOTXORPEN 10
#define R2_NOP 11
#define R2_MERGENOTPEN 12
#define R2_COPYPEN 13
#define R2_MERGEPENNOT 14
#define R2_MERGEPEN 15
#define R2_WHITE 16

// 窗口属性常量
#define EASYX_EX_SHOWCONSOLE 1
#define EASYX_EX_NOCLOSE 2
#define EASYX_EX_NOMINIMIZE 4
#define EASYX_EX_DBLCLKS 8

// 消息类别常量
#define EASYX_EX_MOUSE 1
#define EASYX_EX_KEY 2
#define EASYX_EX_CHAR 4
#define EASYX_EX_WINDOW 8

// 旧版窗口属性常量 (graphics.h)
#define EASYX_SHOWCONSOLE 1
#define EASYX_NOCLOSE 2
#define EASYX_NOMINIMIZE 4
#define EASYX_EW_SHOWCONSOLE 1
#define EASYX_EW_NOCLOSE 2
#define EASYX_EW_NOMINIMIZE 4
#define EASYX_EW_DBLCLKS 8

// 旧版消息类别常量 (graphics.h)
#define EASYX_EM_MOUSE 1
#define EASYX_EM_KEY 2
#define EASYX_EM_CHAR 4
#define EASYX_EM_WINDOW 8

// 旧版填充样式常量 (graphics.h)
#define EASYX_NULL_FILL 0
#define EASYX_EMPTY_FILL 0
#define EASYX_SOLID_FILL 1

#ifdef __cplusplus
extern "C"
{
#endif

    // 图形窗口相关函数
    HWND easyx_initgraph(int width, int height, int flag);
    void easyx_closegraph();

    // 图形环境相关函数
    void easyx_cleardevice();
    void easyx_setcliprgn(void *hrgn);
    void easyx_clearcliprgn();

    // 坐标和比例相关函数
    void easyx_setorigin(int x, int y);
    void easyx_getaspectratio(float *pxasp, float *pyasp);
    void easyx_setaspectratio(float xasp, float yasp);

    // 绘图模式相关函数
    int easyx_getrop2();
    void easyx_setrop2(int mode);
    int easyx_getpolyfillmode();
    void easyx_setpolyfillmode(int mode);
    void easyx_graphdefaults();

    // 线条样式相关函数
    void easyx_setlinestyle(int style, int thickness, const uint32_t *puserstyle, uint32_t userstylecount);
    void easyx_getlinestyle(uint32_t *pstyle, int *pthickness, uint32_t *puserstyle, uint32_t *puserstylecount);
    uint32_t easyx_getlinestyle_len();

    // 填充样式相关函数
    void easyx_setfillstyle(int style, long hatch, const void *ppattern);
    void easyx_getfillstyle(int *pstyle, long *phatch, void **pppattern);
    void easyx_setfillstyle_pattern(const uint8_t *ppattern8x8);

    // 颜色相关函数
    uint32_t easyx_getlinecolor();
    void easyx_setlinecolor(uint32_t color);
    uint32_t easyx_gettextcolor();
    void easyx_settextcolor(uint32_t color);
    uint32_t easyx_getfillcolor();
    void easyx_setfillcolor(uint32_t color);
    uint32_t easyx_getbkcolor();
    void easyx_setbkcolor(uint32_t color);
    int easyx_getbkmode();
    void easyx_setbkmode(int mode);

    // 颜色模型转换函数
    uint32_t easyx_rgb_to_gray(uint32_t rgb);
    void easyx_rgb_to_hsl(uint32_t rgb, float *H, float *S, float *L);
    void easyx_rgb_to_hsv(uint32_t rgb, float *H, float *S, float *V);
    uint32_t easyx_hsl_to_rgb(float H, float S, float L);
    uint32_t easyx_hsv_to_rgb(float H, float S, float V);

    // 绘图相关函数
    uint32_t easyx_getpixel(int x, int y);
    void easyx_putpixel(int x, int y, uint32_t color);
    void easyx_line(int x1, int y1, int x2, int y2);
    void easyx_rectangle(int left, int top, int right, int bottom);
    void easyx_fillrectangle(int left, int top, int right, int bottom);
    void easyx_solidrectangle(int left, int top, int right, int bottom);
    void easyx_clearrectangle(int left, int top, int right, int bottom);
    void easyx_circle(int x, int y, int radius);
    void easyx_fillcircle(int x, int y, int radius);
    void easyx_solidcircle(int x, int y, int radius);
    void easyx_clearcircle(int x, int y, int radius);
    void easyx_ellipse(int left, int top, int right, int bottom);
    void easyx_fillellipse(int left, int top, int right, int bottom);
    void easyx_solidellipse(int left, int top, int right, int bottom);
    void easyx_clearellipse(int left, int top, int right, int bottom);
    void easyx_roundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight);
    void easyx_fillroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight);
    void easyx_solidroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight);
    void easyx_clearroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight);
    void easyx_arc(int left, int top, int right, int bottom, double stangle, double endangle);
    void easyx_pie(int left, int top, int right, int bottom, double stangle, double endangle);
    void easyx_fillpie(int left, int top, int right, int bottom, double stangle, double endangle);
    void easyx_solidpie(int left, int top, int right, int bottom, double stangle, double endangle);
    void easyx_clearpie(int left, int top, int right, int bottom, double stangle, double endangle);
    void easyx_polyline(const void *points, int num);
    void easyx_polygon(const void *points, int num);
    void easyx_fillpolygon(const void *points, int num);
    void easyx_solidpolygon(const void *points, int num);
    void easyx_clearpolygon(const void *points, int num);
    void easyx_polybezier(const void *points, int num);
    void easyx_floodfill(int x, int y, uint32_t color, int filltype);

    // 文本相关函数
    void easyx_outtextxy(int x, int y, const char *str);
    void easyx_outtextxy_char(int x, int y, char c);
    int easyx_textwidth(const char *str);
    int easyx_textwidth_char(char c);
    int easyx_textheight(const char *str);
    int easyx_textheight_char(char c);
    int easyx_drawtext(const char *str, void *pRect, unsigned int uFormat);
    int easyx_drawtext_char(char c, void *pRect, unsigned int uFormat);
    void easyx_settextstyle(int nHeight, int nWidth, const char *lpszFace);
    void easyx_settextstyle_full(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut);
    void easyx_settextstyle_full_ex(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut, unsigned char fbCharSet, unsigned char fbOutPrecision, unsigned char fbClipPrecision, unsigned char fbQuality, unsigned char fbPitchAndFamily);
    void easyx_settextstyle_logfont(void *pLogFont);
    void easyx_gettextstyle(void *pLogFont);

    // 图像相关函数
    void *easyx_create_image(int width, int height);
    void easyx_destroy_image(void *img);
    void easyx_copy_image(void *pDstImg, const void *pSrcImg);
    int easyx_image_getwidth(void *img);
    int easyx_image_getheight(void *img);
    void easyx_image_resize(void *img, int width, int height);
    int easyx_loadimage_file(void *pDstImg, const char *pImgFile, int nWidth, int nHeight, int bResize);
    int easyx_loadimage_resource(void *pDstImg, const char *pResType, const char *pResName, int nWidth, int nHeight, int bResize);
    void easyx_saveimage(const char *pImgFile, const void *pImg);
    void easyx_getimage(void *pDstImg, int srcX, int srcY, int srcWidth, int srcHeight);
    void easyx_putimage(int dstX, int dstY, const void *pSrcImg, uint32_t dwRop);
    void easyx_putimage_part(int dstX, int dstY, int dstWidth, int dstHeight, const void *pSrcImg, int srcX, int srcY, uint32_t dwRop);
    void easyx_rotateimage(void *dstimg, const void *srcimg, double radian, uint32_t bkcolor, int autosize, int highquality);
    void easyx_resize_device(void *pImg, int width, int height);
    uint32_t *easyx_getimagebuffer(const void *pImg);
    void *easyx_getworkingimage();
    void easyx_setworkingimage(void *pImg);
    void *easyx_getimagehdc(const void *pImg);

    // 其他函数
    int easyx_getwidth();
    int easyx_getheight();
    void easyx_beginbatchdraw();
    void easyx_flushbatchdraw();
    void easyx_flushbatchdraw_rect(int left, int top, int right, int bottom);
    void easyx_endbatchdraw();
    void easyx_endbatchdraw_rect(int left, int top, int right, int bottom);
    void easyx_delay(int ms);
    const char *easyx_geteasyxver();
    HWND easyx_gethwnd();

    // 旧版 graphics.h 相关函数

    // 旧版文本相关函数
    void easyx_setfont(int nHeight, int nWidth, const char *lpszFace);
    void easyx_setfont_full(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut);
    void easyx_setfont_full_ex(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut, unsigned char fbCharSet, unsigned char fbOutPrecision, unsigned char fbClipPrecision, unsigned char fbQuality, unsigned char fbPitchAndFamily);
    void easyx_setfont_logfont(void *pLogFont);
    void easyx_getfont(void *pLogFont);

    // 旧版绘图相关函数
    void easyx_bar(int left, int top, int right, int bottom);
    void easyx_bar3d(int left, int top, int right, int bottom, int depth, int topflag);
    void easyx_drawpoly(int numpoints, const int *polypoints);
    void easyx_fillpoly(int numpoints, const int *polypoints);

    // 旧版最大坐标相关函数
    int easyx_getmaxx();
    int easyx_getmaxy();

    // 旧版颜色相关函数
    uint32_t easyx_getcolor();
    void easyx_setcolor(uint32_t color);

    // 旧版光栅操作函数
    void easyx_setwritemode(int mode);

    // 旧版当前位置相关函数
    int easyx_getx();
    int easyx_gety();
    void easyx_moveto(int x, int y);
    void easyx_moverel(int dx, int dy);
    void easyx_lineto(int x, int y);
    void easyx_linerel(int dx, int dy);
    void easyx_outtext(const char *str);
    void easyx_outtext_char(char c);

    // 旧版鼠标相关函数
    int easyx_mousehit();
    void easyx_getmousemsg(void *pMsg);
    int easyx_peekmousemsg(void *pMsg, int bRemoveMsg);
    void easyx_flushmousemsgbuffer();

    // Message Structure
    struct CExMessage
    {
        USHORT message; // The message identifier
        union
        {
            // Data of the mouse message
            struct
            {
                unsigned char ctrl : 1;    // Indicates whether the CTRL key is pressed
                unsigned char shift : 1;   // Indicates whether the SHIFT key is pressed
                unsigned char lbutton : 1; // Indicates whether the left mouse button is pressed
                unsigned char mbutton : 1; // Indicates whether the middle mouse button is pressed
                unsigned char rbutton : 1; // Indicates whether the right mouse button is pressed
                short x;                   // The x-coordinate of the cursor
                short y;                   // The y-coordinate of the cursor
                short wheel;               // The distance the wheel is rotated, expressed in multiples or divisions of 120
            };

            // Data of the key message
            struct
            {
                BYTE vkcode;                // The virtual-key code of the key
                BYTE scancode;              // The scan code of the key. The value depends on the OEM
                unsigned char extended : 1; // Indicates whether the key is an extended key, such as a function key or a key on the numeric keypad. The value is true if the key is an extended key; otherwise, it is false.
                unsigned char prevdown : 1; // Indicates whether the key is previously up or down
            };

            // Data of the char message
            TCHAR ch;

            // Data of the window message
            struct
            {
                WPARAM wParam;
                LPARAM lParam;
            };
        };
    };

    // 消息相关函数
    void easyx_getmessage(struct CExMessage *msg, unsigned char filter);
    int easyx_peekmessage(struct CExMessage *pMsg, unsigned char filter, int removemsg);
    void easyx_flushmessage(unsigned char filter);
    void easyx_setcapture();
    void easyx_releasecapture();

    // 对话框相关函数
    int easyx_inputbox(char *pString, int nMaxCount, const char *pPrompt, const char *pTitle, const char *pDefault, int width, int height, int bOnlyOK);

#ifdef __cplusplus
}
#endif

#endif // EASYX_WRAPPER_H
