// easyx_wrapper.cpp
// C++ 实现，包装 EasyX 库的函数，提供 C 风格接口

#include "easyx_wrapper.h"
#include <string>
#include <windows.h>
#include <tchar.h>
#include "../EasyX_26.1.1/include/easyx.h"
#include "../EasyX_26.1.1/include/graphics.h"

// 字符串转换辅助函数
inline std::basic_string<TCHAR> ansi_to_tstring(const char *str)
{
    if (!str)
        return std::basic_string<TCHAR>();

#ifdef UNICODE
    // 转换为宽字符
    int len = MultiByteToWideChar(CP_UTF8, 0, str, -1, NULL, 0);
    if (len == 0)
        return std::basic_string<TCHAR>();

    std::basic_string<TCHAR> tstr(len, 0);
    MultiByteToWideChar(CP_UTF8, 0, str, -1, reinterpret_cast<LPWSTR>(&tstr[0]), len);
    return tstr;
#else
    // 直接返回 ANSI 字符串
    return std::basic_string<TCHAR>(str);
#endif
}

// 图形窗口相关函数
HWND easyx_initgraph(int width, int height, int flag)
{
    return initgraph(width, height, flag);
}

void easyx_closegraph()
{
    closegraph();
}

// 图形环境相关函数
void easyx_cleardevice()
{
    cleardevice();
}

void easyx_setcliprgn(void *hrgn)
{
    setcliprgn(reinterpret_cast<HRGN>(hrgn));
}

void easyx_clearcliprgn()
{
    clearcliprgn();
}

// 坐标和比例相关函数
void easyx_setorigin(int x, int y)
{
    setorigin(x, y);
}

void easyx_getaspectratio(float *pxasp, float *pyasp)
{
    getaspectratio(pxasp, pyasp);
}

void easyx_setaspectratio(float xasp, float yasp)
{
    setaspectratio(xasp, yasp);
}

// 绘图模式相关函数
int easyx_getrop2()
{
    return getrop2();
}

void easyx_setrop2(int mode)
{
    setrop2(mode);
}

int easyx_getpolyfillmode()
{
    return getpolyfillmode();
}

void easyx_setpolyfillmode(int mode)
{
    setpolyfillmode(mode);
}

void easyx_graphdefaults()
{
    graphdefaults();
}

// 线条样式相关函数
void easyx_setlinestyle(int style, int thickness, const uint32_t *puserstyle, uint32_t userstylecount)
{
    setlinestyle(style, thickness, reinterpret_cast<const DWORD *>(puserstyle), static_cast<DWORD>(userstylecount));
}

void easyx_getlinestyle(uint32_t *pstyle, int *pthickness, uint32_t *puserstyle, uint32_t *puserstylecount)
{
    LINESTYLE linestyle;
    getlinestyle(&linestyle);

    // 填充 style 和 thickness
    if (pstyle)
    {
        *pstyle = static_cast<uint32_t>(linestyle.style);
    }

    if (pthickness)
    {
        *pthickness = static_cast<int>(linestyle.thickness);
    }

    // 处理 userstyle 和 userstylecount
    if (puserstylecount)
    {
        *puserstylecount = static_cast<uint32_t>(linestyle.userstylecount);
    }

    if (puserstyle && linestyle.puserstyle && linestyle.userstylecount > 0)
    {
        // 复制用户样式数据
        memcpy(puserstyle, linestyle.puserstyle, linestyle.userstylecount * sizeof(uint32_t));
    }
}

uint32_t easyx_getlinestyle_len()
{
    LINESTYLE linestyle;

    getlinestyle(&linestyle);

    return linestyle.userstylecount;
}

// 填充样式相关函数
void easyx_setfillstyle(int style, long hatch, const void *ppattern)
{
    setfillstyle(style, hatch, reinterpret_cast<const IMAGE *>(ppattern));
}

void easyx_getfillstyle(int *pstyle, long *phatch, void **pppattern)
{
    FILLSTYLE style;
    getfillstyle(&style);

    if (pstyle)
    {
        *pstyle = style.style;
    }

    if (phatch)
    {
        *phatch = style.hatch;
    }

    if (pppattern)
    {
        *pppattern = const_cast<void *>(reinterpret_cast<const void *>(style.ppattern));
    }
}

void easyx_setfillstyle_pattern(const uint8_t *ppattern8x8)
{
    setfillstyle(ppattern8x8);
}

// 颜色相关函数
uint32_t easyx_getlinecolor()
{
    return getlinecolor();
}

void easyx_setlinecolor(uint32_t color)
{
    setlinecolor(color);
}

uint32_t easyx_gettextcolor()
{
    return gettextcolor();
}

void easyx_settextcolor(uint32_t color)
{
    settextcolor(color);
}

uint32_t easyx_getfillcolor()
{
    return getfillcolor();
}

void easyx_setfillcolor(uint32_t color)
{
    setfillcolor(color);
}

uint32_t easyx_getbkcolor()
{
    return getbkcolor();
}

void easyx_setbkcolor(uint32_t color)
{
    setbkcolor(color);
}

int easyx_getbkmode()
{
    return getbkmode();
}

void easyx_setbkmode(int mode)
{
    setbkmode(mode);
}

// 颜色模型转换函数
uint32_t easyx_rgb_to_gray(uint32_t rgb)
{
    return RGBtoGRAY(rgb);
}

void easyx_rgb_to_hsl(uint32_t rgb, float *H, float *S, float *L)
{
    RGBtoHSL(rgb, H, S, L);
}

void easyx_rgb_to_hsv(uint32_t rgb, float *H, float *S, float *V)
{
    RGBtoHSV(rgb, H, S, V);
}

uint32_t easyx_hsl_to_rgb(float H, float S, float L)
{
    return HSLtoRGB(H, S, L);
}

uint32_t easyx_hsv_to_rgb(float H, float S, float V)
{
    return HSVtoRGB(H, S, V);
}

// 绘图相关函数
uint32_t easyx_getpixel(int x, int y)
{
    return getpixel(x, y);
}

void easyx_putpixel(int x, int y, uint32_t color)
{
    putpixel(x, y, color);
}

void easyx_line(int x1, int y1, int x2, int y2)
{
    line(x1, y1, x2, y2);
}

void easyx_rectangle(int left, int top, int right, int bottom)
{
    rectangle(left, top, right, bottom);
}

void easyx_fillrectangle(int left, int top, int right, int bottom)
{
    fillrectangle(left, top, right, bottom);
}

void easyx_solidrectangle(int left, int top, int right, int bottom)
{
    solidrectangle(left, top, right, bottom);
}

void easyx_clearrectangle(int left, int top, int right, int bottom)
{
    clearrectangle(left, top, right, bottom);
}

void easyx_circle(int x, int y, int radius)
{
    circle(x, y, radius);
}

void easyx_fillcircle(int x, int y, int radius)
{
    fillcircle(x, y, radius);
}

void easyx_solidcircle(int x, int y, int radius)
{
    solidcircle(x, y, radius);
}

void easyx_clearcircle(int x, int y, int radius)
{
    clearcircle(x, y, radius);
}

void easyx_ellipse(int left, int top, int right, int bottom)
{
    ellipse(left, top, right, bottom);
}

void easyx_fillellipse(int left, int top, int right, int bottom)
{
    fillellipse(left, top, right, bottom);
}

void easyx_solidellipse(int left, int top, int right, int bottom)
{
    solidellipse(left, top, right, bottom);
}

void easyx_clearellipse(int left, int top, int right, int bottom)
{
    clearellipse(left, top, right, bottom);
}

void easyx_roundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight)
{
    roundrect(left, top, right, bottom, ellipsewidth, ellipseheight);
}

void easyx_fillroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight)
{
    fillroundrect(left, top, right, bottom, ellipsewidth, ellipseheight);
}

void easyx_solidroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight)
{
    solidroundrect(left, top, right, bottom, ellipsewidth, ellipseheight);
}

void easyx_clearroundrect(int left, int top, int right, int bottom, int ellipsewidth, int ellipseheight)
{
    clearroundrect(left, top, right, bottom, ellipsewidth, ellipseheight);
}

void easyx_arc(int left, int top, int right, int bottom, double stangle, double endangle)
{
    arc(left, top, right, bottom, stangle, endangle);
}

void easyx_pie(int left, int top, int right, int bottom, double stangle, double endangle)
{
    pie(left, top, right, bottom, stangle, endangle);
}

void easyx_fillpie(int left, int top, int right, int bottom, double stangle, double endangle)
{
    fillpie(left, top, right, bottom, stangle, endangle);
}

void easyx_solidpie(int left, int top, int right, int bottom, double stangle, double endangle)
{
    solidpie(left, top, right, bottom, stangle, endangle);
}

void easyx_clearpie(int left, int top, int right, int bottom, double stangle, double endangle)
{
    clearpie(left, top, right, bottom, stangle, endangle);
}

void easyx_polyline(const void *points, int num)
{
    polyline(reinterpret_cast<const POINT *>(points), num);
}

void easyx_polygon(const void *points, int num)
{
    polygon(reinterpret_cast<const POINT *>(points), num);
}

void easyx_fillpolygon(const void *points, int num)
{
    fillpolygon(reinterpret_cast<const POINT *>(points), num);
}

void easyx_solidpolygon(const void *points, int num)
{
    solidpolygon(reinterpret_cast<const POINT *>(points), num);
}

void easyx_clearpolygon(const void *points, int num)
{
    clearpolygon(reinterpret_cast<const POINT *>(points), num);
}

void easyx_polybezier(const void *points, int num)
{
    polybezier(reinterpret_cast<const POINT *>(points), num);
}

void easyx_floodfill(int x, int y, uint32_t color, int filltype)
{
    floodfill(x, y, color, filltype);
}

// 文本相关函数
void easyx_outtextxy(int x, int y, const char *str)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(str);
    outtextxy(x, y, tstr.c_str());
}

void easyx_outtextxy_char(int x, int y, char c)
{
    outtextxy(x, y, static_cast<TCHAR>(c));
}

int easyx_textwidth(const char *str)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(str);
    return textwidth(tstr.c_str());
}

int easyx_textwidth_char(char c)
{
    return textwidth(static_cast<TCHAR>(c));
}

int easyx_textheight(const char *str)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(str);
    return textheight(tstr.c_str());
}

int easyx_textheight_char(char c)
{
    return textheight(static_cast<TCHAR>(c));
}

int easyx_drawtext(const char *str, void *pRect, unsigned int uFormat)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(str);
    return drawtext(tstr.c_str(), reinterpret_cast<RECT *>(pRect), uFormat);
}

int easyx_drawtext_char(char c, void *pRect, unsigned int uFormat)
{
    return drawtext(static_cast<TCHAR>(c), reinterpret_cast<RECT *>(pRect), uFormat);
}

void easyx_settextstyle(int nHeight, int nWidth, const char *lpszFace)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    settextstyle(nHeight, nWidth, tstr.c_str());
}

void easyx_settextstyle_full(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    settextstyle(nHeight, nWidth, tstr.c_str(), nEscapement, nOrientation, nWeight, bItalic != 0, bUnderline != 0, bStrikeOut != 0);
}

void easyx_settextstyle_full_ex(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut, unsigned char fbCharSet, unsigned char fbOutPrecision, unsigned char fbClipPrecision, unsigned char fbQuality, unsigned char fbPitchAndFamily)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    settextstyle(nHeight, nWidth, tstr.c_str(), nEscapement, nOrientation, nWeight, bItalic != 0, bUnderline != 0, bStrikeOut != 0, fbCharSet, fbOutPrecision, fbClipPrecision, fbQuality, fbPitchAndFamily);
}

void easyx_settextstyle_logfont(void *pLogFont)
{
    settextstyle(reinterpret_cast<LOGFONT *>(pLogFont));
}

void easyx_gettextstyle(void *pLogFont)
{
    gettextstyle(reinterpret_cast<LOGFONT *>(pLogFont));
}

// 图像相关函数
void *easyx_create_image(int width, int height)
{
    return new IMAGE(width, height);
}

void easyx_destroy_image(void *img)
{
    delete reinterpret_cast<IMAGE *>(img);
}

void easyx_copy_image(void *pDstImg, const void *pSrcImg)
{
    *reinterpret_cast<IMAGE *>(pDstImg) = *reinterpret_cast<const IMAGE *>(pSrcImg);
}

int easyx_image_getwidth(void *img)
{
    return reinterpret_cast<IMAGE *>(img)->getwidth();
}

int easyx_image_getheight(void *img)
{
    return reinterpret_cast<IMAGE *>(img)->getheight();
}

void easyx_image_resize(void *img, int width, int height)
{
    reinterpret_cast<IMAGE *>(img)->Resize(width, height);
}

int easyx_loadimage_file(void *pDstImg, const char *pImgFile, int nWidth, int nHeight, int bResize)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(pImgFile);
    return loadimage(reinterpret_cast<IMAGE *>(pDstImg), tstr.c_str(), nWidth, nHeight, bResize != 0);
}

void easyx_saveimage(const char *pImgFile, const void *pImg)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(pImgFile);
    saveimage(tstr.c_str(), reinterpret_cast<const IMAGE *>(pImg));
}

void easyx_getimage(void *pDstImg, int srcX, int srcY, int srcWidth, int srcHeight)
{
    getimage(reinterpret_cast<IMAGE *>(pDstImg), srcX, srcY, srcWidth, srcHeight);
}

void easyx_putimage(int dstX, int dstY, const void *pSrcImg, uint32_t dwRop)
{
    putimage(dstX, dstY, reinterpret_cast<const IMAGE *>(pSrcImg), dwRop);
}

void easyx_putimage_part(int dstX, int dstY, int dstWidth, int dstHeight, const void *pSrcImg, int srcX, int srcY, uint32_t dwRop)
{
    putimage(dstX, dstY, dstWidth, dstHeight, reinterpret_cast<const IMAGE *>(pSrcImg), srcX, srcY, dwRop);
}

void easyx_rotateimage(void *dstimg, const void *srcimg, double radian, uint32_t bkcolor, int autosize, int highquality)
{
    rotateimage(reinterpret_cast<IMAGE *>(dstimg), reinterpret_cast<const IMAGE *>(srcimg), radian, bkcolor, autosize != 0, highquality != 0);
}

uint32_t *easyx_getimagebuffer(const void *pImg)
{
    return reinterpret_cast<uint32_t *>(GetImageBuffer(reinterpret_cast<const IMAGE *>(pImg)));
}

void *easyx_getworkingimage()
{
    return GetWorkingImage();
}

void easyx_setworkingimage(void *pImg)
{
    SetWorkingImage(reinterpret_cast<IMAGE *>(pImg));
}

int easyx_loadimage_resource(void *pDstImg, const char *pResType, const char *pResName, int nWidth, int nHeight, int bResize)
{
    std::basic_string<TCHAR> tresType = ansi_to_tstring(pResType);
    std::basic_string<TCHAR> tresName = ansi_to_tstring(pResName);
    return loadimage(reinterpret_cast<IMAGE *>(pDstImg), tresType.c_str(), tresName.c_str(), nWidth, nHeight, bResize != 0);
}

void easyx_resize_device(void *pImg, int width, int height)
{
    Resize(reinterpret_cast<IMAGE *>(pImg), width, height);
}

void *easyx_getimagehdc(const void *pImg)
{
    return GetImageHDC(reinterpret_cast<const IMAGE *>(pImg));
}

// 其他函数
int easyx_getwidth()
{
    return getwidth();
}

int easyx_getheight()
{
    return getheight();
}

void easyx_beginbatchdraw()
{
    BeginBatchDraw();
}

void easyx_flushbatchdraw()
{
    FlushBatchDraw();
}

void easyx_flushbatchdraw_rect(int left, int top, int right, int bottom)
{
    FlushBatchDraw(left, top, right, bottom);
}

void easyx_endbatchdraw()
{
    EndBatchDraw();
}

void easyx_endbatchdraw_rect(int left, int top, int right, int bottom)
{
    EndBatchDraw(left, top, right, bottom);
}

void easyx_delay(int ms)
{
    Sleep(ms);
}

const char *easyx_geteasyxver()
{
    static char version[64] = {0};
    const TCHAR *tversion = GetEasyXVer();

#ifdef UNICODE
    // 宽字符转UTF-8
    WideCharToMultiByte(CP_UTF8, 0, reinterpret_cast<LPCWCH>(tversion), -1, version, sizeof(version), NULL, NULL);
#else
    // 直接复制
    strncpy_s(version, sizeof(version), tversion, _TRUNCATE);
#endif

    return version;
}

HWND easyx_gethwnd()
{
    return GetHWnd();
}

// 旧版 graphics.h 相关函数实现

// 旧版文本相关函数
void easyx_setfont(int nHeight, int nWidth, const char *lpszFace)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    setfont(nHeight, nWidth, tstr.c_str());
}

void easyx_setfont_full(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    setfont(nHeight, nWidth, tstr.c_str(), nEscapement, nOrientation, nWeight, bItalic != 0, bUnderline != 0, bStrikeOut != 0);
}

void easyx_setfont_full_ex(int nHeight, int nWidth, const char *lpszFace, int nEscapement, int nOrientation, int nWeight, int bItalic, int bUnderline, int bStrikeOut, unsigned char fbCharSet, unsigned char fbOutPrecision, unsigned char fbClipPrecision, unsigned char fbQuality, unsigned char fbPitchAndFamily)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(lpszFace);
    setfont(nHeight, nWidth, tstr.c_str(), nEscapement, nOrientation, nWeight, bItalic != 0, bUnderline != 0, bStrikeOut != 0, fbCharSet, fbOutPrecision, fbClipPrecision, fbQuality, fbPitchAndFamily);
}

void easyx_setfont_logfont(void *pLogFont)
{
    setfont(reinterpret_cast<LOGFONT *>(pLogFont));
}

void easyx_getfont(void *pLogFont)
{
    getfont(reinterpret_cast<LOGFONT *>(pLogFont));
}

// 旧版绘图相关函数
void easyx_bar(int left, int top, int right, int bottom)
{
    bar(left, top, right, bottom);
}

void easyx_bar3d(int left, int top, int right, int bottom, int depth, int topflag)
{
    bar3d(left, top, right, bottom, depth, topflag != 0);
}

void easyx_drawpoly(int numpoints, const int *polypoints)
{
    drawpoly(numpoints, polypoints);
}

void easyx_fillpoly(int numpoints, const int *polypoints)
{
    fillpoly(numpoints, polypoints);
}

// 旧版最大坐标相关函数
int easyx_getmaxx()
{
    return getmaxx();
}

int easyx_getmaxy()
{
    return getmaxy();
}

// 旧版颜色相关函数
uint32_t easyx_getcolor()
{
    return getcolor();
}

void easyx_setcolor(uint32_t color)
{
    setcolor(color);
}

// 旧版光栅操作函数
void easyx_setwritemode(int mode)
{
    setwritemode(mode);
}

// 旧版当前位置相关函数
int easyx_getx()
{
    return getx();
}

int easyx_gety()
{
    return gety();
}

void easyx_moveto(int x, int y)
{
    moveto(x, y);
}

void easyx_moverel(int dx, int dy)
{
    moverel(dx, dy);
}

void easyx_lineto(int x, int y)
{
    lineto(x, y);
}

void easyx_linerel(int dx, int dy)
{
    linerel(dx, dy);
}

void easyx_outtext(const char *str)
{
    std::basic_string<TCHAR> tstr = ansi_to_tstring(str);
    outtext(tstr.c_str());
}

void easyx_outtext_char(char c)
{
    outtext(static_cast<TCHAR>(c));
}

// 旧版鼠标相关函数
int easyx_mousehit()
{
    return MouseHit() ? 1 : 0;
}

void easyx_getmousemsg(void *pMsg)
{
    MOUSEMSG msg = GetMouseMsg();
    memcpy(pMsg, &msg, sizeof(MOUSEMSG));
}

int easyx_peekmousemsg(void *pMsg, int bRemoveMsg)
{
    return PeekMouseMsg(reinterpret_cast<MOUSEMSG *>(pMsg), bRemoveMsg != 0) ? 1 : 0;
}

void easyx_flushmousemsgbuffer()
{
    FlushMouseMsgBuffer();
}

// 消息相关函数
void easyx_getmessage(CExMessage *pMsg, unsigned char filter)
{
    getmessage(reinterpret_cast<ExMessage *>(pMsg), filter);
}

int easyx_peekmessage(CExMessage *pMsg, unsigned char filter, int removemsg)
{
    return peekmessage(reinterpret_cast<ExMessage *>(pMsg), filter, removemsg != 0);
}

void easyx_flushmessage(unsigned char filter)
{
    flushmessage(filter);
}

void easyx_setcapture()
{
    setcapture();
}

void easyx_releasecapture()
{
    releasecapture();
}

// 对话框相关函数
int easyx_inputbox(char *pString, int nMaxCount, const char *pPrompt, const char *pTitle, const char *pDefault, int width, int height, int bOnlyOK)
{
    std::basic_string<TCHAR> tprompt = pPrompt ? ansi_to_tstring(pPrompt) : std::basic_string<TCHAR>();
    std::basic_string<TCHAR> ttitle = pTitle ? ansi_to_tstring(pTitle) : std::basic_string<TCHAR>();
    std::basic_string<TCHAR> tdefault = pDefault ? ansi_to_tstring(pDefault) : std::basic_string<TCHAR>();

    TCHAR *tstring = new TCHAR[nMaxCount];
    bool result = InputBox(tstring, nMaxCount, tprompt.c_str(), ttitle.c_str(), tdefault.c_str(), width, height, bOnlyOK != 0);

    // 在第824行添加
#ifdef UNICODE
    // 宽字符转UTF-8
    WideCharToMultiByte(CP_UTF8, 0, tstring, -1, pString, nMaxCount, NULL, NULL);
#else
    // 直接复制
    strncpy_s(pString, nMaxCount, tstring, _TRUNCATE);
#endif

    delete[] tstring;
    return result ? 1 : 0;
}
