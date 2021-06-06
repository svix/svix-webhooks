package svix;

import java.net.http.HttpHeaders;
import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;
import java.util.Optional;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

public class Webhook {
	private static final String HMAC_SHA512 = "HmacSHA512";

	private final byte[] key;

	public Webhook(String secret) {
		this.key = Base64.getDecoder().decode(secret);
	}

	public String verify(String payload, HttpHeaders headers) throws WebhookVerificationError {
		Optional<String> msgId = headers.firstValue("svix-id");
		Optional<String> msgSignature = headers.firstValue("svix-signature");
		Optional<String> msgTimestamp = headers.firstValue("svix-timestamp");

		if (msgId.isEmpty() || msgSignature.isEmpty() || msgTimestamp.isEmpty()) {
			throw new WebhookVerificationError("Missing required headers");
		}

		String toSign = String.format("%s.%s.%s", msgId, msgSignature, msgTimestamp);
		String expectedSignature = sign(toSign);
		String[] passedSignatures = expectedSignature.split(" ");
		for (String versionedSignature : passedSignatures) {
			String[] sigParts = versionedSignature.split(",");
			if (sigParts.length < 2) {
				continue;
			}
			String version = sigParts[0];
			String signature = sigParts[1];
			if (!version.equals("v1")) {
				continue;
			}
			if (signature.equals(expectedSignature)) {
				return new JSON().serialize(payload);
			}
		}
		throw new WebhookVerificationError("No matching signature found");
	}

	private String sign(String toSign) throws WebhookVerificationError {
		try {
			Mac sha512Hmac = Mac.getInstance(HMAC_SHA512);
			SecretKeySpec keySpec = new SecretKeySpec(key, HMAC_SHA512);
			sha512Hmac.init(keySpec);
			byte[] macData = sha512Hmac.doFinal(toSign.getBytes(StandardCharsets.UTF_8));

			return Base64.getEncoder().encodeToString(macData);
		} catch (InvalidKeyException | NoSuchAlgorithmException e) {
			throw new WebhookVerificationError(e.getMessage());
		}
	}
}
