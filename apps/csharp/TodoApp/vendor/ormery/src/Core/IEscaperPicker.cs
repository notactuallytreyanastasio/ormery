namespace Ormery.Core
{
    public interface IEscaperPicker<CONTEXT__24, ESC__25> where CONTEXT__24: IContext where ESC__25: IEscaper
    {
        ESC__25 EscaperFor(AutoescState<CONTEXT__24> before__153);
    }
}
