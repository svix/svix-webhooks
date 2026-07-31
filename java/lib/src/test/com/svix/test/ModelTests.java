package com.svix.test;

import static org.junit.Assert.*;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.svix.models.*;
import java.util.HashMap;
import org.junit.Test;

public class ModelTests {
  @Test
  public void toJsonWorks() throws JsonProcessingException {
    AppPortalAccessOut model = new AppPortalAccessOut().token("asd");
    assertEquals("{\"token\":\"asd\"}", model.toJson());
  }

  @Test
  public void fromJsonWorks() throws JsonProcessingException {
    AppPortalAccessOut expectedModel = new AppPortalAccessOut().token("asd");
    AppPortalAccessOut model = AppPortalAccessOut.fromJson("{\"token\":\"asd\"}");
    assertEquals(expectedModel, model);
  }

  @Test
  public void structEnumWithNoExtraFields() throws JsonProcessingException {
    String jsonString =
        "{\"name\":\"mendy\",\"uid\":\"very unique\",\"metadata\":{},"
            + "\"type\":\"generic-webhook\",\"config\":{}}";
    IngestSourceIn sourceIn =
        new IngestSourceIn()
            .name("mendy")
            .uid("very unique")
            .metadata(new HashMap<>())
            .config(new IngestSourceInConfig.GenericWebhook());

    assertEquals(jsonString, sourceIn.toJson());
    assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
  }

  @Test
  public void structEnumWithFields() throws JsonProcessingException {
    String jsonString =
        "{\"name\":\"name\",\"uid\":\"uuiidd\",\"metadata\":{},"
            + "\"type\":\"cron\",\"config\":{\"schedule\":\"* * * * *\",\"payload\":\"cool\",\"contentType\":\"asd\"}}";
    IngestSourceIn sourceIn =
        new IngestSourceIn()
            .name("name")
            .uid("uuiidd")
            .metadata(new HashMap<>())
            .config(new IngestSourceInConfig.Cron(
                new CronConfig().contentType("asd").payload("cool").schedule("* * * * *")
            )
        );
    assertEquals(jsonString, sourceIn.toJson());
    assertEquals(IngestSourceIn.fromJson(jsonString), sourceIn);
  }

  @Test
  public void readStructEnumField() throws JsonProcessingException {
    String jsonString =
        "{\"name\":\"name\",\"uid\":\"uuiidd\",\"type\":\"cron\",\"config\":{\"contentType\":\"asd\",\"payload\":\"cool\",\"schedule\":\"*"
            + " * * * *\"}}";
    IngestSourceIn sourceIn = IngestSourceIn.fromJson(jsonString);
    assertTrue(sourceIn.getConfig() instanceof IngestSourceInConfig.Cron);
    assertEquals(
        "asd", ((IngestSourceInConfig.Cron) sourceIn.getConfig()).getCron().getContentType());
  }
}
