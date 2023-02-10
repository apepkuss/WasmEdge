// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2019-2022 Second State INC

#pragma once

#include "common/errcode.h"
#include "host/mock/log.h"
#include "runtime/callingframe.h"
#include "runtime/hostfunc.h"

namespace WasmEdge {
namespace Host {
namespace WasiCryptoMock {

using namespace std::literals;

namespace Common {
class ArrayOutputLen : public Runtime::HostFunction<ArrayOutputLen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class ArrayOutputPull : public Runtime::HostFunction<ArrayOutputPull> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class OptionsOpen : public Runtime::HostFunction<OptionsOpen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class OptionsClose : public Runtime::HostFunction<OptionsClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class OptionsSet : public Runtime::HostFunction<OptionsSet> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class OptionsSetU64 : public Runtime::HostFunction<OptionsSetU64> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint64_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class OptionsSetGuestBuffer
    : public Runtime::HostFunction<OptionsSetGuestBuffer> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretsManagerOpen : public Runtime::HostFunction<SecretsManagerOpen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretsManagerClose : public Runtime::HostFunction<SecretsManagerClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretsManagerInvalidate
    : public Runtime::HostFunction<SecretsManagerInvalidate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint64_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};
} // namespace Common

namespace AsymmetricCommon {
class KeypairGenerate : public Runtime::HostFunction<KeypairGenerate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairImport : public Runtime::HostFunction<KeypairImport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairGenerateManaged
    : public Runtime::HostFunction<KeypairGenerateManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairStoreManaged : public Runtime::HostFunction<KeypairStoreManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairReplaceManaged
    : public Runtime::HostFunction<KeypairReplaceManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairId : public Runtime::HostFunction<KeypairId> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairFromId : public Runtime::HostFunction<KeypairFromId> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint64_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairFromPkAndSk : public Runtime::HostFunction<KeypairFromPkAndSk> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairExport : public Runtime::HostFunction<KeypairExport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairPublickey : public Runtime::HostFunction<KeypairPublickey> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairSecretkey : public Runtime::HostFunction<KeypairSecretkey> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeypairClose : public Runtime::HostFunction<KeypairClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class PublickeyImport : public Runtime::HostFunction<PublickeyImport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class PublickeyExport : public Runtime::HostFunction<PublickeyExport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class PublickeyVerify : public Runtime::HostFunction<PublickeyVerify> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class PublickeyFromSecretkey
    : public Runtime::HostFunction<PublickeyFromSecretkey> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class PublickeyClose : public Runtime::HostFunction<PublickeyClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretkeyImport : public Runtime::HostFunction<SecretkeyImport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretkeyExport : public Runtime::HostFunction<SecretkeyExport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class SecretkeyClose : public Runtime::HostFunction<SecretkeyClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};
} // namespace AsymmetricCommon

namespace Kx {
class Dh : public Runtime::HostFunction<Dh> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class Encapsulate : public Runtime::HostFunction<Encapsulate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class Decapsulate : public Runtime::HostFunction<Decapsulate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};
} // namespace Kx

namespace Signatures {
class Export : public Runtime::HostFunction<Export> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class Import : public Runtime::HostFunction<Import> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateOpen : public Runtime::HostFunction<StateOpen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateUpdate : public Runtime::HostFunction<StateUpdate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateSign : public Runtime::HostFunction<StateSign> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateClose : public Runtime::HostFunction<StateClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class VerificationStateOpen
    : public Runtime::HostFunction<VerificationStateOpen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class VerificationStateUpdate
    : public Runtime::HostFunction<VerificationStateUpdate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class VerificationStateVerify
    : public Runtime::HostFunction<VerificationStateVerify> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class VerificationStateClose
    : public Runtime::HostFunction<VerificationStateClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class Close : public Runtime::HostFunction<Close> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

} // namespace Signatures

namespace Symmetric {
class KeyGenerate : public Runtime::HostFunction<KeyGenerate> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyImport : public Runtime::HostFunction<KeyImport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyExport : public Runtime::HostFunction<KeyExport> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyClose : public Runtime::HostFunction<KeyClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyGenerateManaged : public Runtime::HostFunction<KeyGenerateManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyStoreManaged : public Runtime::HostFunction<KeyStoreManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyReplaceManaged : public Runtime::HostFunction<KeyReplaceManaged> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, int32_t,
                        int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyId : public Runtime::HostFunction<KeyId> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class KeyFromId : public Runtime::HostFunction<KeyFromId> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint64_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateOpen : public Runtime::HostFunction<StateOpen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, uint32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateClone : public Runtime::HostFunction<StateClone> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateOptionsGet : public Runtime::HostFunction<StateOptionsGet> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateOptionsGetU64 : public Runtime::HostFunction<StateOptionsGetU64> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateClose : public Runtime::HostFunction<StateClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateAbsorb : public Runtime::HostFunction<StateAbsorb> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateSqueeze : public Runtime::HostFunction<StateSqueeze> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateSqueezeTag : public Runtime::HostFunction<StateSqueezeTag> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateSqueezeKey : public Runtime::HostFunction<StateSqueezeKey> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateMaxTagLen : public Runtime::HostFunction<StateMaxTagLen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateEncrypt : public Runtime::HostFunction<StateEncrypt> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateEncryptDetached
    : public Runtime::HostFunction<StateEncryptDetached> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateDecrypt : public Runtime::HostFunction<StateDecrypt> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateDecryptDetached
    : public Runtime::HostFunction<StateDecryptDetached> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t, uint32_t, uint32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class StateRatchet : public Runtime::HostFunction<StateRatchet> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class TagLen : public Runtime::HostFunction<TagLen> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class TagPull : public Runtime::HostFunction<TagPull> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t, uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class TagVerify : public Runtime::HostFunction<TagVerify> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t, uint32_t,
                        uint32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};

class TagClose : public Runtime::HostFunction<TagClose> {
public:
  Expect<uint32_t> body(const Runtime::CallingFrame &, int32_t) {
    printPluginMock("WASI-Crypto"sv);
    return 1U;
  }
};
} // namespace Symmetric

} // namespace WasiCryptoMock
} // namespace Host
} // namespace WasmEdge
