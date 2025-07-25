// FILE: src/lib/init.ts
(BigInt.prototype as any).toJSON = function () {
    return this.toString();
};
