package ormery.core;
public final class Subsidiary {
    public final Delegate delegate;
    public final Codec codec;
    public static final class Builder {
        Delegate delegate;
        public Builder delegate(Delegate delegate) {
            this.delegate = delegate;
            return this;
        }
        Codec codec;
        public Builder codec(Codec codec) {
            this.codec = codec;
            return this;
        }
        public Subsidiary build() {
            return new Subsidiary(delegate, codec);
        }
    }
    public Subsidiary(Delegate delegate__192, Codec codec__193) {
        this.delegate = delegate__192;
        this.codec = codec__193;
    }
    public Delegate getDelegate() {
        return this.delegate;
    }
    public Codec getCodec() {
        return this.codec;
    }
}
